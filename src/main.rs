mod ppm;
use ppm::*;
mod vector;
mod matrix;
mod geometry;
mod ray_tracer;

use vector::Vector;
use matrix::Matrix;
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
    let fov = 40.0 * (consts::PI / 180.0); 

    let mut buffer = PPM::new(image.width, image.height);

    let aspect_ratio = image.width as f32 / image.height as f32;

    println!("Aspect ratio: {}", aspect_ratio);

    let sphere1 = Sphere::new(Vector::vec3(255.0, 0.0, 255.0), Vector::vec3(0.0, 0.3, -2.3), 0.3);
    let sphere2 = Sphere::new(Vector::vec3(20.0, 165.0, 201.0), Vector::vec3(0.8, 0.0, -2.8), 0.4);
    let sphere3 = Sphere::new(Vector::vec3(201.0, 103.0, 4.0), Vector::vec3(-0.2, 0.0, -1.3), 0.1);

    let mut scene: Vec<Box<dyn RayIntersect>> = Vec::new();
    scene.push(Box::new(sphere1));
    scene.push(Box::new(sphere2));
    scene.push(Box::new(sphere3));

    let origin = Vector::vec3(0.0, 0.0, 0.0);
    println!("vector: {} ", origin);

    for x in 0..image.width {
            let p_x = (2.0 * ((x as f32 + 0.5) / image.width as f32) - 1.0) * aspect_ratio * (fov * 0.5).tan(); 
        for y in 0..image.height {
            let p_y = (1.0 - 2.0 * (y as f32 + 0.5) / image.width as f32) * (fov * 0.5).tan(); 

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

    let m_1 = Matrix::from_vector(
        Vector::vec4(2.0, 7.0, 0.0, 6.0),
        Vector::vec4(7.0, 8.0, 13.0, 2.0),
        Vector::vec4(8.0, 6.0, 0.0, 9.0),
        Vector::vec4(0.0, -2.0, 5.0, -1.0),
    );

    let det = m_1.determinant();

    // let m_2 = Matrix::from_vector(
    //     Vector::vec4(1.0, 7.0, 3.0, 6.0),
    //     Vector::vec4(5.0, 2.0, 5.0, 1.0),
    //     Vector::vec4(7.0, 3.0, 0.0, 7.0),
    //     Vector::vec4(3.0, 4.0, 3.0, 2.0),
    // );

    // let m_3 = m_1 * m_2;

    // let matrix = Matrix::identity();


    println!("original: {} ", det);

    buffer.write_file("image.ppm").expect("Failed Writing File");
}
