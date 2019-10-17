#![allow(dead_code)]

use crate::vector::Vector;
use std::f32;
use std::f32::consts;

pub struct Vertex {
    pub pos: Vector
}

impl Vertex {
    pub fn new(pos: Vector) -> Self {
        Self {
            pos: pos
        }
    }
}

pub struct Mesh {
    pub vertices:Vec<Vertex>,
    pub indices:Vec<u32>,
    pub num_tris:u32
}

pub struct TriangleIntersectResult {
    pub u: f32,
    pub v: f32, 
    pub t: f32
}

pub fn intersect_triangle(ray_origin: Vector, ray_dir: Vector, v_0: Vector, v_1: Vector, v_2:Vector) -> Option<TriangleIntersectResult> {
    let v_01 = v_1 - v_0;
    let v_02 = v_2 - v_0;

    let p = ray_dir.vec3_cross(v_02);
    let det = v_01.vec3_dot(p);

    if det.abs() < f32::EPSILON {
        return None;
    }

    let inv_det = 1.0 / det;

    let t_vec = ray_origin - v_0;
    let u = t_vec.vec3_dot(p) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q_vec = t_vec.vec3_cross(v_01);
    let v = ray_dir.vec3_dot(q_vec) * inv_det;

    if v < 0.0 || v + u > 1.0 {
        return None;
    }

    let t = v_02.vec3_dot(q_vec) * inv_det;

    let result = TriangleIntersectResult{u: u, v: v, t: t};

    Some(result)
}

pub fn create_triangle() -> Mesh {
	let mut vertices = Vec::new();
    let mut indices = Vec::new();
    vertices.push(Vertex::new(Vector::vec3(-1.0, -1.0, -5.0)));
    vertices.push(Vertex::new(Vector::vec3(1.0, -1.0, -5.0)));
    vertices.push(Vertex::new(Vector::vec3(0.0, 1.0, -5.0)));

    indices.push(0);
    indices.push(1);
    indices.push(2);

    Mesh {
        vertices: vertices,
        indices: indices,
        num_tris: 1
    }
}

pub fn create_sphere(radius: f32, slices: u32, stacks: u32) -> Mesh {
    let top_vertex = Vertex::new(Vector::vec3(0.0, radius, -5.0));
	let bottom_vertex = Vertex::new(Vector::vec3(0.0, -radius, -5.0));

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

			let vertex = Vertex::new(Vector::vec3(
                radius * phi.sin() * theta.cos(),
                radius * phi.cos(),
                radius * phi.sin() * theta.sin() - 5.0
                )
            );

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
        num_tris: tris
    }
}