use crate::Vector;
use crate::ray_tracer::*;
use crate::scene::*;
use crate::Stats;
use crate::RenderSettings;
use crate::matrix::Matrix;
use std::f32;
use std::f32::consts;
use crate::math::*;
use rand::Rng;

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
    // pub radius: f32,
    // pub samples: f32,
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
    pub specular: Vector,
    pub roughness: f32,
    pub ior: f32,
    pub transmission: f32
}

impl Material {
    pub fn new(albedo: Vector, specular: Vector, roughness: f32, ior: f32, transmission: f32) -> Self {
        Self {
            albedo: albedo,
            specular: specular,
            roughness: roughness,
            ior: ior,
            transmission: transmission
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
            material: material,
        }
    }
}

pub fn calculate_color(data: ShadingData, dir: Vector, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, stats: & mut Stats) -> Vector {
    let mut diffuse = Vector::vec3(0.0, 0.0, 0.0);
    let mut specular = Vector::vec3(0.0, 0.0, 0.0);

    let lights = &scene.lights;

    for i in 0..lights.len() {
        match &lights[i] {
            Lights::Directional(light) => {  
                let l = -(light.direction.vec3_normalize());
                match trace(data.position + data.normal * 0.0001, l, &scene.scene_objects, f32::INFINITY, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                    None => {
                        let v = -dir;
                        let n = data.normal;

                        compute_lighting(data.material.roughness, data.material.specular, n, v, l, 1.0, light.brightness, light.color, &mut diffuse, &mut specular);
                    },
                    _ => ()
                }
            },
            Lights::Point(light) => {
                let mut l = light.position - data.position;
                let distance = l.vec3_length();
                l /= distance;      
                match trace(data.position + data.normal * 0.0001, l, &scene.scene_objects, distance, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                    None => {
                        let v = -dir;
                        let n = data.normal;

                        let falloff = 4.0 * consts::PI * distance * distance;
                        compute_lighting(data.material.roughness, data.material.specular, n, v, l, falloff, light.brightness, light.color, &mut diffuse, &mut specular);
                    },
                    _ => ()
                }
            }
        }
    }

    let indirect_light = compute_indirect_light(dir, &data, scene, current_ray_depth, settings, ray_type, stats);
    let color = data.material.albedo / consts::PI * (diffuse + indirect_light.0) + specular + indirect_light.1;
    Vector::vec3(clamp(color.x(), 0.0, 1.0), clamp(color.y(), 0.0, 1.0), clamp(color.z(), 0.0, 1.0))
}

fn compute_lighting(roughness: f32, specular_color: Vector, n: Vector, v: Vector, l: Vector, falloff: f32, brightness: f32, light_color: Vector, diffuse: &mut Vector, specular: &mut Vector) {
    let a2 = roughness * roughness;

    let h =  (v + l).vec3_normalize();
    let n_o_v = n.vec3_dot(v).abs() + 1e-5;
    let l_o_h = clamp(l.vec3_dot(h), 0.0, 1.0);
    let n_o_h = clamp(n.vec3_dot(h), 0.0, 1.0);
    let n_o_l = clamp(n.vec3_dot(l), 0.0, 1.0);

    let F = schlick_fresnel_aprx(l_o_h, specular_color);
    let D = ggx_distribution(n_o_h, a2);
    let G = smith_for_ggx(n_o_l, n_o_v, a2);
    let brdf = F * G * D;

    let diffuse_term = Vector::vec3(1.0, 1.0, 1.0) - F;
    let energy = (light_color * brightness) / falloff * n_o_l;
    *specular += brdf * energy;
    *diffuse += diffuse_term * energy;
}

#[inline]
fn ggx_distribution(n_dot_h: f32, a: f32) -> f32 {
    let a2 = a * a;
    let d = (n_dot_h * a2 - n_dot_h) * n_dot_h + 1.0;
    a2 / (consts::PI * d * d)
}

#[inline]
fn smith_for_ggx(n_dot_l: f32, n_dot_v: f32, a: f32) -> f32 {
    let a2 = a * a;
    let lambda_l = n_dot_v * ((-n_dot_l * a2 + n_dot_l) * n_dot_l + a2).sqrt();
    let lambda_v = n_dot_l * ((-n_dot_v * a2 + n_dot_v) * n_dot_v + a2).sqrt();
    0.5 / (lambda_l + lambda_v)  
}

#[inline]
fn schlick_fresnel_aprx(l_dot_h: f32, spec_color: Vector) -> Vector {
    spec_color + (spec_color - 1.0) * (1.0 - l_dot_h).powi(5)
}

fn compute_indirect_light(dir: Vector, data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, stats: & mut Stats) -> (Vector, Vector) {
    let mut indirect_diffuse = Vector::vec3(0.0, 0.0, 0.0);
    let mut indirect_specular = Vector::vec3(0.0, 0.0, 0.0);

    if current_ray_depth < settings.max_ray_depth {
        let t;
        let n = data.normal;
        if n.x().abs() > n.y().abs() {
            t = Vector::vec3(n.z(), 0.0, -n.x()) / (n.x() * n.x() + n.z() * n.z()).sqrt();
        } else {
            t = Vector::vec3(0.0, -n.z(), n.y()) / (n.y() * n.y() + n.z() * n.z()).sqrt();
        }
    
        let b = n.vec3_cross(t);
    
        let tbn = Matrix::from_vector(
            t, n, b, Vector::vec4(0.0, 0.0, 0.0, 1.0)
        );

        compute_indirect_diffuse(data, scene, current_ray_depth, settings, ray_type, &tbn, &mut indirect_diffuse, stats);
        compute_indirect_specular(dir, data, scene, current_ray_depth, settings, ray_type, &tbn, &mut indirect_specular, stats);
    }

    (indirect_diffuse, indirect_specular)
}

fn compute_indirect_diffuse(data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, tbn: &Matrix, diffuse: & mut Vector, stats: & mut Stats) {
    if settings.diffuse_samples > 0 && (ray_type == RayType::CameraRay || ray_type == RayType::DiffuseRay) {
        let mut samples = settings.diffuse_samples;
        let n = data.normal;

        if current_ray_depth > 0 {
            samples = 1;
        }

        for _ in 0..samples {
            let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
            let rand2 = rand::thread_rng().gen_range(0.0, 1.0);
        
            let sample = sample_hemisphere_cosine_weighted(rand1, rand2);
        
            let dir = (sample.0 * *tbn).vec3_normalize();
            let pdf = sample.1;
            *diffuse += (cast_ray(data.position + dir * 0.0001, dir, scene, current_ray_depth + 1, settings, RayType::DiffuseRay, stats) / pdf) * clamp(dir.vec3_dot(n), 0.0, 1.0);
        }
    
        *diffuse /= samples as f32;
    }
}

fn compute_indirect_specular(dir: Vector, data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, tbn: &Matrix, specular: & mut Vector, stats: & mut Stats) {

    if settings.specular_samples > 0 && (ray_type == RayType::CameraRay || ray_type == RayType::SpecularRay) {
        let mut samples = settings.specular_samples;

        if current_ray_depth > 0 {
            samples = 1;
        }

        let V = -dir;
        let n = data.normal;
        let a2 = data.material.roughness * data.material.roughness;

        for _ in 0..samples {
            let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
            let rand2 = rand::thread_rng().gen_range(0.0, 1.0);
        
            let sample = importance_sample_ggx(rand1, rand2, a2);
        
            let H = (sample.0 * *tbn).vec3_normalize();
			let L = (H * 2.0 * V.vec3_dot(H)) - V;

            let pdf = sample.1;

            let n_o_v = n.vec3_dot(V).abs();
            let n_o_l = clamp(n.vec3_dot(L), 0.0, 1.0);

            if n_o_v == 0.0 || n_o_l == 0.0 {
                continue;
            }

            let light_color = (cast_ray(data.position + L * 0.0001, L, scene, current_ray_depth + 1, settings, RayType::SpecularRay, stats) / pdf) * n_o_l;

            let l_o_h = clamp(L.vec3_dot(H), 0.0, 1.0);
            let n_o_h = clamp(n.vec3_dot(H), 0.0, 1.0);

            let F = schlick_fresnel_aprx(l_o_h, data.material.specular);
            let D = ggx_distribution(n_o_h, a2);
            let G = smith_for_ggx(n_o_l, n_o_v, a2);
            let brdf = F * D * G * light_color;

            *specular += brdf;
        }
    
        *specular /= samples as f32;
    }
}

#[inline]
fn sample_hemisphere_uniform(rand1: f32, rand2:f32) -> (Vector, f32) {
    let sin_theta = (1.0 - rand1 * rand1).sqrt();
    let phi = 2.0 * consts::PI * rand2;

    let x = sin_theta * phi.cos();
    let z = sin_theta * phi.sin();
    let pdf = 1.0 / (2.0 * consts::PI);
    (Vector::vec3(x, sin_theta, z), pdf)
}

#[inline]
fn sample_hemisphere_cosine_weighted(rand1: f32, rand2:f32) -> (Vector, f32) {
    let sin2_theta  = rand1;
    let cos2_theta = 1.0 - sin2_theta;
    let sin_theta = sin2_theta.sqrt();
    let cos_theta = cos2_theta.sqrt();

    let phi = 2.0 * consts::PI * rand2;

    let x = sin_theta * phi.cos();
    let z = sin_theta * phi.sin();

    let pdf = cos_theta / consts::PI;

    (Vector::vec3(x, cos_theta, z), pdf)
}

#[inline]
fn importance_sample_ggx(rand1: f32, rand2:f32, roughness: f32) -> (Vector, f32) {
	let a2 = roughness * roughness;

	let phi = 2.0 * consts::PI * rand1;
	let cos_theta = ((1.0 - rand2) / ( 1.0 + (a2 - 1.0) * rand2 )).sqrt();
	let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

	let x = sin_theta * phi.cos();
	let z = sin_theta * phi.sin();
	
	let d = (cos_theta * a2 - cos_theta) * cos_theta + 1.0;
	let D = a2 / (consts::PI * d * d);
	let pdf = D * cos_theta;

    (Vector::vec3(x, cos_theta, z), pdf)
}