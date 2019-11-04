use crate::Vector;
use crate::ray_tracer::trace;
use crate::scene::*;
use std::f32;

pub struct DirectionalLight {
    pub direction: Vector,
    pub brightness: f32,
    pub color: Vector
} 

pub struct PointLight {
    pub position: Vector,
    pub brightness: f32,
    pub color: Vector,
    pub range: f32,
    pub attenuation: Vector
}

impl DirectionalLight {
    pub fn new(dir: Vector, brightness: f32, color: Vector) -> Self {
        Self {
            direction: dir,
            brightness: brightness,
            color: color
        }
    } 
}

pub enum Lights {
    Directional(DirectionalLight),
    Point(PointLight)
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Vector
}

impl Material {
    pub fn new(albedo: Vector) -> Self {
        Self {
            albedo: albedo
        }
    }
}

pub struct ShadingData {
    position: Vector,
    normal: Vector,
    textureCoord: Vector,
    material: Material
}

impl ShadingData {
    pub fn new (position: Vector, normal: Vector, textureCoord: Vector, material: Material) -> Self {
        Self {
            position: position,
            normal: normal,
            textureCoord: textureCoord,
            material: material
        }
    }
}

pub fn calculate_color(data: ShadingData, origin: Vector, lights: &[Lights], scene_object: &[SceneObject]) -> Vector {
    let mut diffuse = Vector::vec3(0.0, 0.0, 0.0);

    for i in 0..lights.len() {
        match &lights[i] {
            Lights::Directional(light) => {  
                let light_dir = -light.direction;      
                match trace(data.position + data.normal * 0.0001, light_dir, scene_object, f32::INFINITY) {
                    None => {
                        diffuse += light.color * light_dir.vec3_dot(data.normal.vec3_normalize()).max(0.0) * light.brightness;
                    },
                    _ => ()
                }
                // diffuse += light.color * light_dir.vec3_dot(data.normal.vec3_normalize()).max(0.0) * light.brightness;
            },
            Lights::Point(point) => ()
        }
    }

    data.material.albedo * diffuse + data.material.albedo * Vector::vec3(0.1, 0.1, 0.15)
    // Vector::vec3(data.normal.x() + 1.0, data.normal.y() + 1.0, data.normal.z() + 1.0) * 0.5
}