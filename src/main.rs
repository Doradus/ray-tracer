mod ppm;
use ppm::*;
mod vec3;
mod geometry;
mod ray_tracer;

use vec3::Vec3;
use geometry::Sphere;
use geometry::RayIntersect;
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
    let fov = consts::PI / 2.0; 

    let mut buffer = PPM::new(image.width, image.height);

    let aspect_ratio = image.width as f32 / image.height as f32;

    println!("Aspect ratio: {}", aspect_ratio);

    let sphere1 = Sphere::new(Vec3::new(201.0, 103.0, 4.0), Vec3::new(-0.2, 0.0, -0.6), 0.3);
    let sphere2 = Sphere::new(Vec3::new(20.0, 165.0, 201.0), Vec3::new(0.3, 0.0, -0.8), 0.4);

    let mut scene: Vec<Box<dyn RayIntersect>> = Vec::new();
    scene.push(Box::new(sphere1));
    scene.push(Box::new(sphere2));

    let origin = Vec3::new(0.0, 0.0, 0.0);

    for x in 0..image.width {
        for y in 0..image.height {
            let p_x = (2.0 * ((x as f32 + 0.5) / image.width as f32) - 1.0) * aspect_ratio * (fov / 2.0).tan(); 
            let p_y = (2.0 * ((y as f32 + 0.5) / image.width as f32) - 1.0) * (fov / 2.0).tan(); 

            let dir = Vec3::new(p_x, p_y, -1.0) - origin;
            let dir = dir.normalize();

            let ray_color = cast_ray(origin, dir, &scene);

            let color = RGB {
                r: ray_color.x as u8,
                g: ray_color.y as u8,
                b: ray_color.z as u8
            };

            buffer.set_pixel(x, y, color);
        }
    } 



    buffer.write_file("image.ppm").expect("Failed Writing File");
}
