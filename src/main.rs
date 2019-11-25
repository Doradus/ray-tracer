mod vector;
mod matrix;
mod geometry;
mod shading;
mod ray_tracer;
mod scene;
mod math;
mod test_scenes;

use scene::*;
use test_scenes::*;
use vector::Vector;
use std::time::Instant;
use ray_tracer::cast_ray;
use std::{f32::consts, fmt};
use image;

struct RenderSettings {
    width:u32,
    height:u32,
    max_ray_depth: u32
}

impl RenderSettings {
    fn new(width: u32, height: u32, ray_depth: u32) -> Self {
        Self {width: width, height: height, max_ray_depth: ray_depth}
    }
}

pub struct Stats {
    pub num_rays_shot: u128,
    pub num_tringle_tests: u128,
    pub num_triangles_intersected: u128,
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
    let settings = RenderSettings::new(640, 360, 2);

    let mut buffer: image::RgbImage = image::ImageBuffer::new(settings.width, settings.height);

    let scene = gi_test();

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

            let ray_color = cast_ray(origin, dir.vec3_normalize(), &scene, 0, stats);

            let pixel = buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb(
                [(ray_color.x() * 255.0) as u8, (ray_color.y() * 255.0) as u8, (ray_color.z() * 255.0) as u8]);
        }
    }
}

fn write_to_file(buffer: &image::RgbImage) {
    buffer.save("image.png").unwrap();
}
