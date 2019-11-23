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
    let settings = RenderSettings::new(1280, 720, 2);

    let buffer = UnsafeRgbaImage::new(image::RgbImage::new(settings.width, settings.height));

    let scene = transmission_test();

    let now = Instant::now();

    let cell_width = settings.width / 2;
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
            let mut stats3 = Stats {..Default::default()};
            render(0 * cell_width, 1 * cell_height, cell_width, cell_height, &buffer, settings, &scene, &mut stats3);
        });

        s.spawn(|_| {
            let mut stats4 = Stats {..Default::default()};
            render(1 * cell_width, 1 * cell_height, cell_width, cell_height, &buffer, settings, &scene, &mut stats4);
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
        let p_x = (2.0 * (x as f32 + 0.5) / settings.width as f32 - 1.0) * a; 
        for y in offset_y..end_y {
            let p_y = (1.0 - 2.0 * (y as f32 + 0.5) / settings.height as f32) * scale; 

            let dir = Vector::vec3(p_x, p_y, -1.0);

            let ray_color = cast_ray(origin, dir.vec3_normalize(), &scene, stats);

            buffer.put_pixel(x, y, image::Rgb([(ray_color.x() * 255.0) as u8, (ray_color.y() * 255.0) as u8, (ray_color.z() * 255.0) as u8]));
        }
    }
}

fn write_to_file(buffer: & UnsafeRgbaImage) {
    buffer.as_ref().save("image.png").unwrap();
}
