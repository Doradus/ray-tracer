mod vector_simd;
mod matrix;
mod geometry;
mod shading;
mod ray_tracer;
mod scene;
mod math;
mod test_scenes;
mod bvh;
mod camera;

use scene::*;
use test_scenes::*;
use vector_simd::Vector;
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
    pub aa_samples: u32,
    pub background_color: Vector
}

impl RenderSettings {
    fn new(width: u32, height: u32, ray_depth: u32, diffuse_samples: u32, specular_samples: u32, aa_samples: u32, background_color: Vector) -> Self {
        Self {width: width, height: height, max_ray_depth: ray_depth, diffuse_samples: diffuse_samples, specular_samples: specular_samples, aa_samples: aa_samples, background_color: background_color}
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
    let settings = RenderSettings::new(1280, 720, 2, 4, 4, 7, Vector::vec3(0.86, 0.92, 1.0));
    //let settings = RenderSettings::new(1280, 720, 2, 3, 0, 8, Vector::vec3(0.0, 0.0, 0.0));
    let buffer = UnsafeRgbaImage::new(image::RgbImage::new(settings.width, settings.height));

    let scene = spehres();

    let max_threads = num_cpus::get();
    println!("threads: {}", max_threads);

    let mut thread_info = Vec::new();
    for y in 0..settings.height as u32 {
        for x in 0..settings.width as u32 {
            thread_info.push(Vector::vec2(x as f32, y as f32));
        }
    }

    let num_render_jobs = settings.width * settings.height;
    let render_job_counter = AtomicUsize::new(0);
    let threads_spawned = AtomicUsize::new(0);

    let now = Instant::now();

    let origin = Vector::vec3(0.0, 0.0, 0.0) * scene.camera.to_world;
    let aspect_ratio = settings.width as f32 / settings.height as f32;
    let fov = 40.0 * (consts::PI / 180.0); 
    let scale = (fov * 0.5).tan();

    crossbeam_utils::thread::scope(|s| {
        for _ in 0..max_threads {
            s.spawn(|_| {   
                let mut stats = Stats {..Default::default()};
                let thread_num = threads_spawned.fetch_add(1, Ordering::Relaxed);
                loop {
                    let i = render_job_counter.fetch_add(1, Ordering::Relaxed);

                    if i >= num_render_jobs as usize {
                        break;
                    }

                    render(thread_info[i], &buffer, origin, aspect_ratio, scale, settings, &scene, &mut stats);

                }
                println!("thread: {}, num triangle intersects: {}", thread_num, stats.num_tringle_tests);
            });       
        };
    }).unwrap();

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    println!("render time {}", end);

    write_to_file(&buffer);
}

fn render(info: Vector, buffer: & UnsafeRgbaImage, origin: Vector, aspect_ratio: f32, scale: f32, settings: RenderSettings, scene: &SceneData, stats: &mut Stats ) {
    let a = aspect_ratio * scale;
    let x = info.x();
    let y = info.y();

    let mut color = Vector::vec3(0.0, 0.0, 0.0);
    let sample_points = get_aa_distribution(settings.aa_samples);
    for i in 0..sample_points.len() {
        let p_x = (2.0 * (x + sample_points[i].0) / settings.width as f32 - 1.0) * a; 
        let p_y = (1.0 - 2.0 * (y + sample_points[i].1) / settings.height as f32) * scale; 
        let dir = Vector::vec3(p_x, p_y, -1.0)  * scene.camera.to_world;
        let ray_dir = dir - origin;
        color += cast_ray(origin, ray_dir.vec3_normalize(), &scene, 0, settings, RayType::CameraRay, stats);
    }

    color /= sample_points.len() as f32;
    let output = color.clamp(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(1.0, 1.0, 1.0)) * 255.0;
    let (r, g, b, a2) = output.into();
    buffer.put_pixel(x as u32, y as u32, image::Rgb([r as u8, g as u8, b as u8]));
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
