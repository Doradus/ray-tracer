mod ppm;
use ppm::*;
mod vector;
mod matrix;
mod geometry;
mod shading;
mod ray_tracer;
mod scene;

use scene::*;
use vector::Vector;
use matrix::Matrix;
use geometry::*;
use shading::Material;
use std::time::Instant;
use ray_tracer::cast_ray;
use std::f32::consts;

struct Info {
    width:u32,
    height:u32
}

impl Info {
    fn new(width: u32, height: u32) -> Self {
        Self {width: width, height: height}
    }
}

fn main() {
    let info = Info::new(1024, 1024);

    let mut buffer = PPM::new(info.width, info.height);

//    let triangle = create_scene_object(
//         create_triangle(),
//         Material::new(Vector::vec3(255.0, 0.0, 0.0)),
//         Vector::vec3(0.0, 0.0, -5.0),
//         Vector::vec3(1.0, 1.0, 1.0),
//         Vector::vec3(0.0, 0.0, 0.0)
//     );

    let sphere = create_scene_object(
        create_sphere(1.0, 40, 20),
        Material::new(Vector::vec3(255.0, 0.0, 0.0)),
        Vector::vec3(0.0, 0.0, -5.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );
   
    let plane = create_scene_object(
        create_plane(5.0, 5.0, 5, 5),
        Material::new(Vector::vec3(255.0, 0.0, 0.0)),
        Vector::vec3(0.0, -1.0, -5.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let scene_objects = vec![sphere, plane];

    let scene = SceneData {
        scene_objects
    };

    let now = Instant::now();

    render(& mut buffer, info, &scene);

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;

    write_to_file(&buffer);
    println!("image generated in: {} seconds", end);
}

fn render(buffer: & mut PPM, info: Info, scene: &SceneData) {
    let origin = Vector::vec3(0.0, 0.0, 0.0);
    let aspect_ratio = info.width as f32 / info.height as f32;
    let fov = 40.0 * (consts::PI / 180.0); 


    for x in 0..info.width {
        let p_x = (2.0 * ((x as f32 + 0.5) / info.width as f32) - 1.0) * aspect_ratio * (fov * 0.5).tan(); 
        for y in 0..info.height {
            let p_y = (1.0 - 2.0 * (y as f32 + 0.5) / info.width as f32) * (fov * 0.5).tan(); 

            let dir = Vector::vec3(p_x, p_y, -1.0);
            let dir = dir.vec3_normalize();

            let ray_color = cast_ray(origin, dir, &scene);

            let color = RGB {
                r: ray_color.x() as u8,
                g: ray_color.y() as u8,
                b: ray_color.z() as u8
            };

            buffer.set_pixel(x, y, color);
        }
    }
}

fn create_scene_object(mesh: Mesh, material: Material, position:Vector, scale: Vector, rotation: Vector) -> SceneObject {
    let scale_matrix = Matrix::scaling_matrix(scale);
    // let rotation_matrix = Matrix::scaling_matrix(rotation);
    let translation_matrix = Matrix::translation_matrix(position);

    let world_matrix = scale_matrix * translation_matrix;
    let inv_world = world_matrix.inverse().transpose();
    // let transpose = inv_world.transpose();

    let mut transformed_vertices = Vec::new();
    for i in 0..mesh.vertices.len() {
        let vertex = Vertex::new(mesh.vertices[i].pos * world_matrix, mesh.vertices[i].normal * inv_world);       
        transformed_vertices.push(vertex);
    }

    let mesh_data = Mesh {
        vertices: transformed_vertices,
        indices: mesh.indices,
        num_tris: mesh.num_tris
    };

    SceneObject::new(mesh_data, material)
}

fn write_to_file(buffer: &PPM) {
    buffer.write_file("image.ppm").expect("Failed Writing File");
}
