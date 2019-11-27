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

use std::cell::UnsafeCell;

pub struct UnsafeRgbaImage(UnsafeCell<image::RgbImage>);

impl UnsafeRgbaImage {
    pub fn new(img: image::RgbImage) -> Self {
        Self(UnsafeCell::new(img))
    }

    pub fn as_ref(&self) -> &image::RgbImage {
        unsafe { self.0.get().as_ref() }.unwrap()
    }

    pub fn put_pixel(&self, x: u32, y: u32, pixel: image::Rgb<u8>) {
        unsafe { self.0.get().as_mut() }
            .unwrap()
            .put_pixel(x, y, pixel)
    }
}

unsafe impl Sync for UnsafeRgbaImage {}

#[derive(Clone, Copy, Debug)]
pub struct RenderSettings {
    pub width:u32,
    pub height:u32,
    pub max_ray_depth: u32,
    pub diffuse_samples: u32,
    pub aa_samples: u32
}

impl RenderSettings {
    fn new(width: u32, height: u32, ray_depth: u32, diffuse_samples: u32, aa_samples: u32) -> Self {
        Self {width: width, height: height, max_ray_depth: ray_depth, diffuse_samples: diffuse_samples, aa_samples: aa_samples}
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
    let settings = RenderSettings::new(1280, 720, 2, 32, 3);
    let buffer = UnsafeRgbaImage::new(image::RgbImage::new(settings.width, settings.height));

    let scene = gi_test();

    let now = Instant::now();

    let cell_width = settings.width / 3;
    let cell_height = settings.height / 2;


    crossbeam_utils::thread::scope(|s| {
        s.spawn(|_| {
            let mut stats1 = Stats {..Default::default()};
            render(0 * cell_width, 0 * cell_height, cell_width, cell_height, &buffer, settings, &scene, &mut stats1);
        });

        s.spawn(|_| {
            let mut stats2 = Stats {..Default::default()};
            render(1 * cell_width, 0 * cell_height, cell_width, cell_height, &buffer, settings, &scene, &mut stats2);
        });

        s.spawn(|_| {
            let mut stats2 = Stats {..Default::default()};
            render(2 * cell_width, 0 * cell_height, cell_width + 1, cell_height, &buffer, settings, &scene, &mut stats2);
        });

        s.spawn(|_| {
            let mut stats3 = Stats {..Default::default()};
            render(0 * cell_width, 1 * cell_height, cell_width, cell_height, &buffer, settings, &scene, &mut stats3);
        });

        s.spawn(|_| {
            let mut stats4 = Stats {..Default::default()};
            render(1 * cell_width, 1 * cell_height, cell_width, cell_height, &buffer, settings, &scene, &mut stats4);
        });

        s.spawn(|_| {
            let mut stats4 = Stats {..Default::default()};
            render(2 * cell_width, 1 * cell_height, cell_width + 1, cell_height, &buffer, settings, &scene, &mut stats4);
        });
    }).unwrap();


    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    write_to_file(&buffer);

    println!("{}", end);
}

fn render(offset_x: u32, offset_y: u32, width: u32, height: u32, buffer: & UnsafeRgbaImage, settings: RenderSettings, scene: &SceneData, stats: &mut Stats) {
    let origin = Vector::vec3(0.0, 0.0, 0.0);
    let aspect_ratio = settings.width as f32 / settings.height as f32;
    let fov = 40.0 * (consts::PI / 180.0); 

    let scale = (fov * 0.5).tan();
    let a = aspect_ratio * scale;

    let end_x = width + offset_x;
    let end_y = height + offset_y;
    for x in offset_x..end_x {
        for y in offset_y..end_y {

            let mut color = Vector::vec3(0.0, 0.0, 0.0);
            let sample_points = get_aa_distribution(settings.aa_samples);
            for i in 0..sample_points.len() {
                let p_x = (2.0 * (x as f32 + sample_points[i].0) / settings.width as f32 - 1.0) * a; 
                let p_y = (1.0 - 2.0 * (y as f32 + sample_points[i].1) / settings.height as f32) * scale; 
                let dir = Vector::vec3(p_x, p_y, -1.0);

                color += cast_ray(origin, dir.vec3_normalize(), &scene, 0, settings, stats);
            }

            color /= sample_points.len() as f32;
            buffer.put_pixel(x, y, image::Rgb([(color.x() * 255.0) as u8, (color.y() * 255.0) as u8, (color.z() * 255.0) as u8]));
        }
    }
}

fn get_aa_distribution(samples: u32) -> Vec<(f32, f32)> {
    let mut sample_pos = Vec::new();
    for x in 0..samples {
        for y in 0..samples {
            let ratio = 1.0 / samples as f32;
            let sample_pos_x = x as f32 * ratio + ratio * 0.5;
            let sample_pos_y = y as f32 * ratio + ratio * 0.5;

            sample_pos.push((sample_pos_x, sample_pos_y))
        }
    }

    sample_pos
}

fn write_to_file(buffer: & UnsafeRgbaImage) {
    buffer.as_ref().save("image.png").unwrap();
}
