use crate::Vector;
use crate::ray_tracer::trace;
use crate::scene::*;
use crate::Stats;
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

impl PointLight {
    pub fn new(pos: Vector, brightness: f32, color: Vector, range: f32, attenuation: Vector) -> Self {
        Self {
            position: pos,
            brightness: brightness,
            color: color,
            range: range,
            attenuation: attenuation
        }
    }
}

pub enum Lights {
    Directional(DirectionalLight),
    Point(PointLight)
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Vector,
    pub roughness: f32
}

impl Material {
    pub fn new(albedo: Vector, roughness: f32) -> Self {
        Self {
            albedo: albedo,
            roughness: roughness
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

pub fn calculate_color(data: ShadingData, origin: Vector, lights: &[Lights], scene_object: &[SceneObject], stats: & mut Stats) -> Vector {
    let mut diffuse = Vector::vec3(0.0, 0.0, 0.0);

    for i in 0..lights.len() {
        match &lights[i] {
            Lights::Directional(light) => {  
                let light_dir = -light.direction;      
                match trace(data.position + data.normal * 0.0001, light_dir, scene_object, f32::INFINITY, stats) {
                    None => {
                        diffuse += light.color * light_dir.vec3_dot(data.normal.vec3_normalize()).max(0.0) * light.brightness;
                    },
                    _ => ()
                }
            },
            Lights::Point(light) => {
                let light_dir = light.position - data.position;
                let distance = light_dir.vec3_length();
                let light_dir = light_dir.vec3_normalize();      
                match trace(data.position + data.normal * 0.0001, light_dir, scene_object, distance, stats) {
                    None => {
                        let falloff = 1.0 / light.attenuation.vec3_dot(Vector::vec3(1.0, distance, distance * distance));
                        diffuse += light.color * light_dir.vec3_dot(data.normal.vec3_normalize()).max(0.0) * light.brightness * falloff;
                    },
                    _ => ()
                }
            }
        }
    }

    data.material.albedo * diffuse + data.material.albedo * Vector::vec3(0.3, 0.3, 0.35)
}