#![allow(dead_code)]

use crate::vector_simd::{Axis, Vector};
use std::{f32, fmt, mem};
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

pub fn create_triangle() -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    vertices.push(Vertex::new(Vector::vec3(-1.0, -1.0, 0.0),Vector::vec3(0.0, 0.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(1.0, -1.0, 0.0), Vector::vec3(0.0, 0.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(0.0, 1.0, 0.0), Vector::vec3(0.0, 0.0, 1.0)));

    indices.push(0);
    indices.push(1);
    indices.push(2);

    Mesh {
        vertices: vertices,
        indices: indices,
        num_tris: 1,
    }
}

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

pub fn create_box(width: f32, height: f32, depth: f32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let half_width = width * 0.5;
    let half_height = height * 0.5;
    let half_depth = depth * 0.5;

    //front
    vertices.push(Vertex::new(Vector::vec3(-half_width, -half_height, half_depth), Vector::vec3(0.0, 0.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, -half_height, half_depth), Vector::vec3(0.0, 0.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, half_height, half_depth), Vector::vec3(0.0, 0.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, half_height, half_depth), Vector::vec3(0.0, 0.0, 1.0)));

    //left
    vertices.push(Vertex::new(Vector::vec3(-half_width, -half_height, half_depth), Vector::vec3(-1.0, 0.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, half_height, half_depth), Vector::vec3(-1.0, 0.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, -half_height, -half_depth), Vector::vec3(-1.0, 0.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, half_height, -half_depth), Vector::vec3(-1.0, 0.0, 0.0)));

    //right
    vertices.push(Vertex::new(Vector::vec3(half_width, -half_height, half_depth), Vector::vec3(1.0, 0.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, half_height, half_depth), Vector::vec3(1.0, 0.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, -half_height, -half_depth), Vector::vec3(1.0, 0.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, half_height, -half_depth), Vector::vec3(1.0, 0.0, 0.0)));

    //back
    vertices.push(Vertex::new(Vector::vec3(-half_width, -half_height, -half_depth), Vector::vec3(0.0, 0.0, -1.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, -half_height, -half_depth), Vector::vec3(0.0, 0.0, -1.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, half_height, -half_depth), Vector::vec3(0.0, 0.0, -1.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, half_height, -half_depth), Vector::vec3(0.0, 0.0, -1.0)));

    //top
    vertices.push(Vertex::new(Vector::vec3(-half_width, half_height, half_depth), Vector::vec3(0.0, 1.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, half_height, half_depth), Vector::vec3(0.0, 1.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, half_height, -half_depth), Vector::vec3(0.0, 1.0, 0.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, half_height, -half_depth), Vector::vec3(0.0, 1.0, 0.0)));

    //bottom
    vertices.push(Vertex::new(Vector::vec3(-half_width, -half_height, half_depth), Vector::vec3(0.0, -1.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, -half_height, half_depth), Vector::vec3(0.0, -1.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(-half_width, -half_height, -half_depth), Vector::vec3(0.0, -1.0, 1.0)));
    vertices.push(Vertex::new(Vector::vec3(half_width, -half_height, -half_depth), Vector::vec3(0.0, -1.0, 1.0)));

    //front
    indices.push(1); indices.push(3); indices.push(0);
    indices.push(2); indices.push(0); indices.push(3);

    //left
    indices.push(4); indices.push(5); indices.push(6);
    indices.push(7); indices.push(6); indices.push(5);

    //right
    indices.push(10); indices.push(11); indices.push(8);
    indices.push(9); indices.push(8); indices.push(11);

    //back
    indices.push(12); indices.push(14); indices.push(13);
    indices.push(15); indices.push(13); indices.push(14);

    //top
    indices.push(17); indices.push(19); indices.push(16);
    indices.push(18); indices.push(16); indices.push(19);

    //bottom
    indices.push(20); indices.push(22); indices.push(21);
    indices.push(23); indices.push(21); indices.push(22);


    Mesh {
        vertices: vertices,
        indices: indices,
        num_tris: 12,
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

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub bounds:[Vector; 2]
}

impl BoundingBox {
    pub fn new() -> Self {
        Self {
            bounds: [Vector::vec3(f32::MAX, f32::MAX, f32::MAX), Vector::vec3(f32::MIN, f32::MIN, f32::MIN)]
        }
    }

    pub fn union(self, other: BoundingBox) -> Self {
        Self {
            bounds: [
                Vector::vec3(
                    self.bounds[0].x().min(other.bounds[0].x()),
                    self.bounds[0].y().min(other.bounds[0].y()),
                    self.bounds[0].z().min(other.bounds[0].z())
                ),
                Vector::vec3(
                    self.bounds[1].x().max(other.bounds[1].x()),
                    self.bounds[1].y().max(other.bounds[1].y()),
                    self.bounds[1].z().max(other.bounds[1].z())
                )
            ]
        }
    }

    pub fn union_from_vector(self, other: Vector) -> Self {
        Self {
            bounds: [
                Vector::vec3(
                    self.bounds[0].x().min(other.x()),
                    self.bounds[0].y().min(other.y()),
                    self.bounds[0].z().min(other.z())
                ),
                Vector::vec3(
                    self.bounds[1].x().max(other.x()),
                    self.bounds[1].y().max(other.y()),
                    self.bounds[1].z().max(other.z())
                )
            ]
        }
    }
    
    pub fn maximum_extent(self)-> u8 {
        let d = self.diagonal();

        if d.x() > d.y() && d.x() > d.z() {
            return 0;
        } else if d.y() > d.z() {
            return 1;
        } else {
            return 2;
        }
    }

    pub fn diagonal(self) -> Vector {
        self.bounds[1] - self.bounds[0]
    }

    pub fn min(self) -> Vector {
        self.bounds[0]
    }

    pub fn max(self) -> Vector {
        self.bounds[1]
    }

    pub fn extend_bounds(&mut self, pos: Vector) {
        if pos.x() < self.bounds[0].x() {self.bounds[0].set_x(pos.x())};
        if pos.y() < self.bounds[0].y() {self.bounds[0].set_y(pos.y())};
        if pos.z() < self.bounds[0].z() {self.bounds[0].set_z(pos.z())};

        if pos.x() > self.bounds[1].x() {self.bounds[1].set_x(pos.x())};
        if pos.y() > self.bounds[1].y() {self.bounds[1].set_y(pos.y())};
        if pos.z() > self.bounds[1].z() {self.bounds[1].set_z(pos.z())};
    }

    pub fn offset(self, point: Vector) -> Vector {
        let mut offset = point - self.bounds[0];

        if self.bounds[0].x() < self.bounds[1].x() {
            offset.set_x(offset.x() / (self.bounds[1].x() - self.bounds[0].x()));
        }

        if self.bounds[0].y() < self.bounds[1].y() {
            offset.set_y(offset.y() / (self.bounds[1].y() - self.bounds[0].y()));
        }

        if self.bounds[0].z() < self.bounds[1].z() {
            offset.set_z(offset.z() / (self.bounds[1].z() - self.bounds[0].z()));
        }
        
        offset
    }

    pub fn surface_area(self) -> f32 {
        let d = self.diagonal();

        2.0 * (d.x() * d.y() + d.x() * d.z() + d.y() * d.z())
    }

    pub fn intersect(&self, ray_origin: Vector, inv_ray_dir: Vector, sign: [i8; 3]) -> bool {
        let sign_x = sign[0] as usize;
        let sign_y = sign[1] as usize;
        let sign_z = sign[2] as usize;

        let min = (Vector::vec3(self.bounds[sign_x].x(), self.bounds[sign_y].y(), self.bounds[sign_z].z()) - ray_origin) * inv_ray_dir;
        let max = (Vector::vec3(self.bounds[1 - sign_x].x(), self.bounds[1 - sign_y].y(), self.bounds[1 - sign_z].z()) - ray_origin) * inv_ray_dir;

        let mut t_min = min.x();
        let mut t_max = max.x();

        let t_y_min = min.y();
        let t_y_max = max.y();

        if (t_min > t_y_max) || (t_y_min > t_max) {
            return false;
        }

        if t_y_min > t_min {
            t_min = t_y_min;
        }

        if t_y_max < t_max {
            t_max = t_y_max;
        }

        let t_z_min = min.z();
        let t_z_max = max.z();


        if (t_min > t_z_max) || (t_z_min > t_max) {
            return false;
        }


        // let mut t_min = (self.bounds[sign_x].x() - ray_origin.x()) * inv_ray_dir.x();
        // let mut t_max = (self.bounds[1 - sign_x].x() - ray_origin.x()) * inv_ray_dir.x();

        // let t_y_min = (self.bounds[sign_y].y() - ray_origin.y()) * inv_ray_dir.y();
        // let t_y_max = (self.bounds[1 - sign_y].y() - ray_origin.y()) * inv_ray_dir.y();

        // if (t_min > t_y_max) || (t_y_min > t_max) {
        //     return false;
        // }

        // if t_y_min > t_min {
        //     t_min = t_y_min;
        // }

        // if t_y_max < t_max {
        //     t_max = t_y_max;
        // }

        // let t_z_min = (self.bounds[sign_z].z() - ray_origin.z()) * inv_ray_dir.z();
        // let t_z_max = (self.bounds[1 - sign_z].z() - ray_origin.z()) * inv_ray_dir.z();


        // if (t_min > t_z_max) || (t_z_min > t_max) {
        //     return false;
        // }

        true
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "min bounds: {},\n max bounds: {}", self.bounds[0], self.bounds[1]
        )
    }
}

pub struct BoundingSpehere {
    pub position: Vector,
    pub radius: f32,
    radius_sqrd: f32,
    pub bounds:Vec<Vector>
}

impl BoundingSpehere {
    pub fn new(pos: Vector, radius: f32) -> Self {
        Self {
            position: pos,
            radius: radius,
            radius_sqrd: radius * radius,
            bounds: vec![Vector::vec3(pos.x(), pos.y(), pos.z()), Vector::vec3(pos.x(), pos.y(), pos.z())]
        }
    }

    pub fn extend_bounds(&mut self, pos: Vector) {
        if pos.x() < self.bounds[0].x() {self.bounds[0].set_x(pos.x())};
        if pos.y() < self.bounds[0].y() {self.bounds[0].set_y(pos.y())};
        if pos.z() < self.bounds[0].z() {self.bounds[0].set_z(pos.z())};

        if pos.x() > self.bounds[1].x() {self.bounds[1].set_x(pos.x())};
        if pos.y() > self.bounds[1].y() {self.bounds[1].set_y(pos.y())};
        if pos.z() > self.bounds[1].z() {self.bounds[1].set_z(pos.z())};

        let x = self.bounds[0].x() - self.bounds[1].x();
        let y = self.bounds[0].x() - self.bounds[1].x();
        let z = self.bounds[0].x() - self.bounds[1].x();

        self.position.set_x(x * 0.5);
        self.position.set_y(y * 0.5);
        self.position.set_z(z * 0.5);

        let radius = x.abs().max(y.abs()).max(z.abs());

        self.radius = radius;
        self.radius_sqrd = radius * radius;
    }

    pub fn intersect(&self, ray_origin: Vector, ray_dir: Vector) -> bool {
        let L = self.position - ray_origin;

        let tca = L.vec3_dot_f32(ray_dir);

        if tca < 0.0 {
            return false;
        }

        let d2 = L.vec3_dot_f32(L) - tca * tca;

        if d2 > self.radius_sqrd {
            return false;            
        }

        let mut t0;
        let t1;

        let thc = (self.radius_sqrd - d2).sqrt(); 
        t0 = tca - thc; 
        t1 = tca + thc; 

        if t0 < 0.0 { 
            t0 = t1;
            if t0 < 0.0 {
                return false;                 
            } 
        }

        true
    }
}


