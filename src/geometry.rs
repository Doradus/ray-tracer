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