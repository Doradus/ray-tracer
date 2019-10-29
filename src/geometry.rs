#![allow(dead_code)]

use crate::vector::Vector;
use std::f32;
use std::f32::consts;

pub struct Vertex {
    pub pos: Vector,
    pub normal: Vector
}

impl Vertex {
    pub fn new(pos: Vector, norm: Vector) -> Self {
        Self { pos: pos, normal: norm }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub num_tris: u32,
}

// pub fn create_triangle() -> Mesh {
//     let mut vertices = Vec::new();
//     let mut indices = Vec::new();
//     vertices.push(Vertex::new(Vector::vec3(-1.0, -1.0, 0.0),Vector::vec3(0.0, 0.0, 1.0), Vector::vec3(255.0, 0.0, 0.0)));
//     vertices.push(Vertex::new(Vector::vec3(1.0, -1.0, 0.0), Vector::vec3(0.0, 0.0, 1.0), Vector::vec3(0.0, 255.0, 0.0)));
//     vertices.push(Vertex::new(Vector::vec3(0.0, 1.0, 0.0), Vector::vec3(0.0, 0.0, 1.0), Vector::vec3(0.0, 0.0, 255.0)));

//     indices.push(0);
//     indices.push(1);
//     indices.push(2);

//     Mesh {
//         vertices: vertices,
//         indices: indices,
//         num_tris: 1,
//     }
// }

pub fn create_plane(width: f32, depth: f32, sub_div_width: u32, sub_div_depth: u32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let tri_count = (sub_div_width - 1) * (sub_div_depth - 1) * 2;

    let half_width = 0.5 * width;
    let half_depth = 0.5 * depth;

    let dx = width / (sub_div_width - 1) as f32;
    let dz = depth / (sub_div_depth - 1) as f32;

    for i in 0..sub_div_depth {
        let iter_depth = i as f32;
        let z = half_depth - iter_depth * dz;
 
        for j in 0..sub_div_width {
            let iter_width = j as f32;
            let x = -half_width + iter_width * dx;

            let vertex = Vertex::new(Vector::vec3(x, 0.0, z), Vector::vec3(0.0, 1.0, 0.0));

            vertices.push(vertex);
        }
    }

    let lines_width = sub_div_width - 1;
    let lines_depths = sub_div_depth - 1;
    for i in 0..lines_width {
        for j in 0..lines_depths {
            indices.push(i * sub_div_depth + j);
            indices.push(i * sub_div_depth + j + 1);
            indices.push((i + 1) * sub_div_depth + j);
            indices.push((i + 1) * sub_div_depth + j);
            indices.push(i * sub_div_depth + j + 1);
            indices.push((i + 1) * sub_div_depth + j + 1);
        }
    }

    Mesh {
        vertices: vertices,
        indices: indices,
        num_tris: tri_count,
    }
}

pub fn create_sphere(radius: f32, slices: u32, stacks: u32) -> Mesh {
    let top_vertex = Vertex::new(Vector::vec3(0.0, radius, 0.0), Vector::vec3(0.0, 1.0, 0.0));
    let bottom_vertex = Vertex::new(Vector::vec3(0.0, -radius, 0.0), Vector::vec3(0.0, -1.0, 0.0));

    let mut vertices = Vec::new();
    vertices.push(top_vertex);

    let mut indices = Vec::new();

    let phi_step = consts::PI / stacks as f32;
    let theta_step = consts::PI * 2.0 / slices as f32;
    let slices_plus_one = slices + 1;

    for i in 1..stacks {
        let phi = phi_step * i as f32;
        for j in 0..slices_plus_one {
            let theta = theta_step * j as f32;

            let position = Vector::vec3(
                radius * phi.sin() * theta.cos(),
                radius * phi.cos(),
                radius * phi.sin() * theta.sin(),
            );

            let normal = position.vec3_normalize(); 
            let vertex = Vertex::new(position, normal);

            vertices.push(vertex);
        }
    }

    vertices.push(bottom_vertex);

    let slices_end = slices + 1;
    for i in 1..slices_end {
        indices.push(0);
        indices.push(i + 1);
        indices.push(i);
    }

    let offset = 1;
    let ring_vertex = slices + 1;
    let stack_end = stacks - 2;

    for i in 0..stack_end {
        for j in 0..slices {
            indices.push(offset + ring_vertex * i + j);
			indices.push(offset + ring_vertex * i + j + 1);
			indices.push(offset + ring_vertex * (i + 1) + j);

			indices.push(offset + ring_vertex * (i + 1) + j);
			indices.push(offset + ring_vertex * i + j + 1);
			indices.push(offset + ring_vertex * (i + 1) + j + 1);
        }
    }

    let bottom_vertex_index = vertices.len() as u32 - 1;
    let offset = bottom_vertex_index - ring_vertex;

    for i in 0..slices {
        indices.push(bottom_vertex_index);
		indices.push(offset + i);
		indices.push(offset + i + 1);
    }

    let tris = (stacks - 2) * slices * 2 + slices * 2;
    Mesh {
        vertices: vertices,
        indices: indices,
        num_tris: tris,
    }
}
