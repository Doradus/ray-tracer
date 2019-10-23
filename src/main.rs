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
    let image = Image::new(1024, 1024);
    let fov = 40.0 * (consts::PI / 180.0); 

    let mut buffer = PPM::new(image.width, image.height);

    let aspect_ratio = image.width as f32 / image.height as f32;

    let sphere = create_sphere(1.0, 20, 10);
    let plane = create_plane(0.5, 0.5, 5, 5);

	let mut plane_vertices = Vec::new();
	let mut plane_indices = Vec::new();

    let mut sphere_vertices = Vec::new();
	let mut sphere_indices = Vec::new();

    let plane_scaling = Matrix::scaling_matrix(Vector::vec3(10.0, 10.0, 10.0));
    let plane_translation = Matrix::translation_matrix(Vector::vec3(0.0, -0.5, -3.5));
    let plane_world = plane_scaling * plane_translation;

    let sphere_scaling = Matrix::scaling_matrix(Vector::vec3(1.0, 1.0, 1.0));
    let sphere_translation = Matrix::translation_matrix(Vector::vec3(0.0, 0.5, -5.0));
    let sphere_world = sphere_scaling * sphere_translation;


    for i in 0..plane.vertices.len() {
        let pos = plane.vertices[i].pos * plane_world;
        let vertex = Vertex::new(pos);
        plane_vertices.push(vertex);
    }

    for i in 0..plane.indices.len() {
        plane_indices.push(plane.indices[i]);
    }

    for i in 0..sphere.vertices.len() {
        let pos = sphere.vertices[i].pos * sphere_world;
        let vertex = Vertex::new(pos);
        sphere_vertices.push(vertex);
    }

    for i in 0..sphere.indices.len() {
        sphere_indices.push(sphere.indices[i]);
    }

    let scaled_plane = Mesh {
        vertices: plane_vertices,
        indices: plane_indices,
        num_tris: plane.num_tris
    };

    let scaled_sphere = Mesh {
        vertices: sphere_vertices,
        indices: sphere_indices,
        num_tris: sphere.num_tris
    };

    let scene_objects = vec![scaled_plane, scaled_sphere];
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
