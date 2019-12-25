// mod vector;
mod vector_simd;
mod matrix;
mod geometry;
mod shading;
mod ray_tracer;
mod scene;
mod math;
mod test_scenes;
mod bvh;

use scene::*;
use test_scenes::*;
use vector_simd::Vector;
// use vector_simd::VectorSimd;
use std::time::Instant;
use ray_tracer::{RayType, cast_ray};
use std::{f32, f32::consts, fmt};
use image;
use std::sync::atomic::{AtomicUsize, Ordering};
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
    pub specular_samples: u32,
    pub aa_samples: u32
}

impl RenderSettings {
    fn new(width: u32, height: u32, ray_depth: u32, diffuse_samples: u32, specular_samples: u32, aa_samples: u32) -> Self {
        Self {width: width, height: height, max_ray_depth: ray_depth, diffuse_samples: diffuse_samples, specular_samples: specular_samples, aa_samples: aa_samples}
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

#[derive(Copy, Clone)]
struct RenderThreadInfo {
    pub offset: (u32, u32),
    pub dimensions: (u32, u32)
}

fn main() {
    let settings = RenderSettings::new(1280, 720, 3, 0, 0, 1);
    let buffer = UnsafeRgbaImage::new(image::RgbImage::new(settings.width, settings.height));

    let scene = gi_test();

    let max_threads = num_cpus::get();
    println!("threads: {}", max_threads);

    let y_divisions = (max_threads as f32).sqrt().floor();
    let x_divisions = (max_threads as f32) / y_divisions;

    let mut thread_info = Vec::new();

    let cell_width = settings.width / (x_divisions as u32);
    let cell_height = settings.height / (y_divisions as u32); 

    for y in 0..x_divisions as u32 {
        for x in 0..x_divisions as u32 {
            thread_info.push(RenderThreadInfo {offset: (x * cell_width, y * cell_height), dimensions: (cell_width, cell_height)});
        }
    }

    let thread_counter = AtomicUsize::new(0);

    let now = Instant::now();
    crossbeam_utils::thread::scope(|s| {
        for _ in 0..max_threads {
            s.spawn(|_| {   
                let mut stats = Stats {..Default::default()};
                let i = thread_counter.fetch_add(1, Ordering::Relaxed);
                render(thread_info[i], &buffer, settings, &scene, &mut stats);
                println!("thread: {}, num triangle intersects: {}", i, stats.num_tringle_tests);
            });       
        };
    }).unwrap();

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    println!("render time {}", end);

    write_to_file(&buffer);
}

fn render(info: RenderThreadInfo, buffer: & UnsafeRgbaImage, settings: RenderSettings, scene: &SceneData, stats: &mut Stats) {
    let origin = Vector::vec3(0.0, 0.0, 0.0);
    let aspect_ratio = settings.width as f32 / settings.height as f32;
    let fov = 40.0 * (consts::PI / 180.0); 

    let scale = (fov * 0.5).tan();
    let a = aspect_ratio * scale;

    let end_x = info.dimensions.0 + info.offset.0;
    let end_y = info.dimensions.1 + info.offset.1;
    for x in info.offset.0..end_x {
        for y in info.offset.1..end_y {

            let mut color = Vector::vec3(0.0, 0.0, 0.0);
            let sample_points = get_aa_distribution(settings.aa_samples);
            for i in 0..sample_points.len() {
                let p_x = (2.0 * (x as f32 + sample_points[i].0) / settings.width as f32 - 1.0) * a; 
                let p_y = (1.0 - 2.0 * (y as f32 + sample_points[i].1) / settings.height as f32) * scale; 
                let dir = Vector::vec3(p_x, p_y, -1.0);

                color += cast_ray(origin, dir.vec3_normalize(), &scene, 0, settings, RayType::CameraRay, stats);
            }

            color /= sample_points.len() as f32;
            color *= 255.0;
            buffer.put_pixel(x, y, image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8]));
        }
    }
}

fn get_aa_distribution(samples: u32) -> Vec<(f32, f32)> {
    let mut sample_pos = Vec::new();
    let ratio = 1.0 / samples as f32;

    for x in 0..samples {
        for y in 0..samples {
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
