use crate::Vector;
use crate::ray_tracer::trace;
use crate::scene::*;
use crate::Stats;
use std::f32;
use std::f32::consts;
use crate::math::*;

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

pub fn calculate_color(data: ShadingData, dir: Vector, lights: &[Lights], scene_object: &[SceneObject], stats: & mut Stats) -> Vector {
    let mut diffuse = Vector::vec3(0.0, 0.0, 0.0);
    let mut specular = Vector::vec3(0.0, 0.0, 0.0);

    for i in 0..lights.len() {
        match &lights[i] {
            Lights::Directional(light) => {  
                let l = -(light.direction.vec3_normalize());
                match trace(data.position + data.normal * 0.0001, l, scene_object, f32::INFINITY, stats) {
                    None => {
                        let a2 = data.material.roughness * data.material.roughness;

                        let v = -(dir.vec3_normalize());
                        let n = data.normal.vec3_normalize();
                        let h =  (v + l).vec3_normalize();
                        let n_o_v = n.vec3_dot(v).abs() + 1e-5;
                        let l_o_h = clamp(l.vec3_dot(h), 0.0, 1.0);
                        let n_o_h = clamp(n.vec3_dot(h), 0.0, 1.0);
                        let n_o_l = clamp(n.vec3_dot(l), 0.0, 1.0);

                        let F = schlick_fresnel_aprx(l_o_h, Vector::vec3(0.9, 0.9, 0.9));
                        let D = ggx_distribution(n_o_h, a2);
                        let G = smith_for_ggx(n_o_l, n_o_v, a2);
                        let brdf = F * G * D;

                        specular += brdf * light.brightness * n_o_l;
                        diffuse += light.color * l.vec3_dot(data.normal.vec3_normalize()).max(0.0) * light.brightness;
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

    let color = data.material.albedo * diffuse + data.material.albedo * Vector::vec3(0.3, 0.3, 0.35) + specular;
    Vector::vec3(clamp(color.x(), 0.0, 1.0), clamp(color.y(), 0.0, 1.0), clamp(color.z(), 0.0, 1.0))
}

fn ggx_distribution(n_dot_h: f32, a: f32) -> f32 {
    let a2 = a.powi(2);
    a2 / (consts::PI * (n_dot_h.powi(2) * (a2 - 1.0) + 1.0).powi(2))
}

fn smith_for_ggx(n_dot_l: f32, n_dot_v: f32, a: f32) -> f32 {
    let a2 = a * a;
    let lambda_l = n_dot_v * ((-n_dot_l * a2 + n_dot_l) * n_dot_l + a2).sqrt();
    let lambda_v = n_dot_l * ((-n_dot_v * a2 + n_dot_v) * n_dot_v + a2).sqrt();
    0.5 / (lambda_l + lambda_v)  
}

fn schlick_fresnel_aprx(l_dot_h: f32, spec_color: Vector) -> Vector {
    spec_color + (spec_color - 1.0) * (1.0 - l_dot_h).powi(5)
}