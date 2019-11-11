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
use shading::*;
use std::time::Instant;
use ray_tracer::cast_ray;
use std::{f32::consts, fmt};
use image;

struct RenderSettings {
    width:u32,
    height:u32,
    ray_depth: u32
}

impl RenderSettings {
    fn new(width: u32, height: u32, ray_depth: u32) -> Self {
        Self {width: width, height: height, ray_depth: ray_depth}
    }
}

pub struct Stats {
    pub num_rays_shot: u32,
    pub num_tringle_tests: u32,
    pub num_triangles_intersected: u32,
    pub render_time: f64
}

impl Default for Stats {
    fn default() -> Stats {
        Stats {
            num_rays_shot: 0,
            num_tringle_tests: 0,
            num_triangles_intersected: 0,
            render_time: 0.0
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "number of rays shot: {},\n number of triangles tested: {},\n number of triangles intersected: {},\n image generated in: {}",
            self.num_rays_shot, self.num_tringle_tests, self.num_triangles_intersected, self.render_time
        )
    }
}

fn main() {
    let mut stats = Stats {..Default::default()};
    let settings = RenderSettings::new(512, 512, 2);

    let mut buffer: image::RgbImage = image::ImageBuffer::new(settings.width, settings.height);

    let sphere = create_scene_object(
        create_sphere(0.5, 40, 20),
        Material::new(Vector::vec3(0.6, 0.6, 0.6), 1.0),
        Vector::vec3(-0.5, 0.0, -3.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let cube = create_scene_object(
        create_box(1.0, 1.0 , 1.0),
        Material::new(Vector::vec3(0.6, 0.6, 0.6), 1.0),
        Vector::vec3(0.75, 0.0, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0 * consts::PI / 180.0, 45.0 * consts::PI / 180.0, 0.0)
    );
   
    let plane = create_scene_object(
        create_plane(5.0, 5.0, 5, 5),
        Material::new(Vector::vec3(0.6, 0.6, 0.6), 1.0),
        Vector::vec3(0.0, -0.5, -4.0),
        Vector::vec3(1.0, 1.0, 1.0),
        Vector::vec3(0.0, 0.0, 0.0)
    );

    let scene_objects = vec![cube, sphere, plane];

    let directional_light = Lights::Directional(DirectionalLight::new(Vector::vec3(-0.3, -0.3, -0.7), 1.0, Vector::vec3(1.0, 1.0, 1.0)));
    let point_light = Lights::Point(PointLight::new(Vector::vec3(0.0, 1.0, -3.5), 1.0, Vector::vec3(1.0, 1.0, 1.0), 2.0, Vector::vec3(0.0, 1.0, 0.0)));
    let lights = vec![point_light, directional_light];

    let scene = SceneData {
        scene_objects,
        lights
    };

    let now = Instant::now();

    render(& mut buffer, settings, &scene, &mut stats);

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    stats.render_time = end;
    write_to_file(&buffer);

    println!("{}", stats);
}

fn render(buffer: & mut image::RgbImage, settings: RenderSettings, scene: &SceneData, stats: &mut Stats) {
    let origin = Vector::vec3(0.0, 0.0, 0.0);
    let aspect_ratio = settings.width as f32 / settings.height as f32;
    let fov = 40.0 * (consts::PI / 180.0); 

    let scale = (fov * 0.5).tan();
    let a = aspect_ratio * scale;

    for x in 0..settings.width {
        let p_x = (2.0 * (x as f32 + 0.5) / settings.width as f32 - 1.0) * a; 
        for y in 0..settings.height {
            let p_y = (1.0 - 2.0 * (y as f32 + 0.5) / settings.height as f32) * scale; 

            let dir = Vector::vec3(p_x, p_y, -1.0);
            let dir = dir.vec3_normalize();

            let ray_color = cast_ray(origin, dir, &scene, stats);

            let pixel = buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb(
                [(ray_color.x() * 255.0) as u8, (ray_color.y() * 255.0) as u8, (ray_color.z() * 255.0) as u8]);
        }
    }
}

fn create_scene_object(mesh: Mesh, material: Material, position:Vector, scale: Vector, rotation: Vector) -> SceneObject {
    let scale_matrix = Matrix::scaling_matrix(scale);
    // let rotation_matrix = Matrix::scaling_matrix(rotation);
    let translation_matrix = Matrix::translation_matrix(position);
    let rotation_matrix = Matrix::roatation_x(rotation.x()) * Matrix::roatation_y(rotation.y());
    let world_matrix = scale_matrix * rotation_matrix * translation_matrix;
    let inv_world = world_matrix.inverse().transpose();

    let mut bounding_box = BoundingBox::new(mesh.vertices[0].pos * world_matrix);
    let mut transformed_vertices = Vec::new();

    for i in 0..mesh.vertices.len() {
        let vertex = Vertex::new(mesh.vertices[i].pos * world_matrix, mesh.vertices[i].normal * inv_world);
        bounding_box.extend_bounds(vertex.pos);   
        transformed_vertices.push(vertex);
    }

    let mesh_data = Mesh {
        vertices: transformed_vertices,
        indices: mesh.indices,
        num_tris: mesh.num_tris
    };

    SceneObject::new(mesh_data, material, bounding_box)
}

fn write_to_file(buffer: &image::RgbImage) {
    buffer.save("image.png").unwrap();
}
