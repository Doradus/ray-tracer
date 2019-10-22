mod ppm;
use ppm::*;
mod vector;
mod matrix;
mod geometry;
mod ray_tracer;

use vector::Vector;
use matrix::Matrix;
use geometry::*;
use std::time::{Duration, Instant};
use ray_tracer::cast_ray;
use std::f32::consts;

struct Image {
    width:u32,
    height:u32
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        Self {width: width, height: height}
    }
}

fn main() {
    let image = Image::new(512, 512);
    let fov = 40.0 * (consts::PI / 180.0); 

    let mut buffer = PPM::new(image.width, image.height);

    let aspect_ratio = image.width as f32 / image.height as f32;

    let mesh = create_sphere(0.75, 20, 10);

	let mut vertices = Vec::new();
	let mut indices = Vec::new();

    let scaling = Matrix::scaling_matrix(Vector::vec3(2.0, 2.0, 2.0));

    let translation = Matrix::translation_matrix(Vector::vec3(0.0, 0.0, -5.0));

    let world = scaling * translation;
    let world_inv = world.inverse();

    let id = world_inv * world;

    println!("world: {}", world);

    println!("world inv: {}", world_inv);

    println!("id: {}", id);

    for i in 0..mesh.vertices.len() {
        let pos = mesh.vertices[i].pos * world;
        let vertex = Vertex::new(pos);
        vertices.push(vertex);
    }

    for i in 0..mesh.indices.len() {
        indices.push(mesh.indices[i]);
    }

    let scaled_sphere = Mesh {
        vertices: vertices,
        indices: indices,
        num_tris: mesh.num_tris
    };



    let scene_objects = vec![create_plane(0.1, 0.1, 1, 1)];
    let origin = Vector::vec3(0.0, 0.0, 0.0);

    let now = Instant::now();
    for x in 0..image.width {
        let p_x = (2.0 * ((x as f32 + 0.5) / image.width as f32) - 1.0) * aspect_ratio * (fov * 0.5).tan(); 
        for y in 0..image.height {
            let p_y = (1.0 - 2.0 * (y as f32 + 0.5) / image.width as f32) * (fov * 0.5).tan(); 

            let dir = Vector::vec3(p_x, p_y, -1.0);
            let dir = dir.vec3_normalize();

            let ray_color = cast_ray(origin, dir, &scene_objects);

            let color = RGB {
                r: ray_color.x() as u8,
                g: ray_color.y() as u8,
                b: ray_color.z() as u8
            };

            buffer.set_pixel(x, y, color);
        }
    }

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    println!("image generated in: {} seconds", end);
    buffer.write_file("image.ppm").expect("Failed Writing File");
}
