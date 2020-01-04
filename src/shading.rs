#![allow(dead_code)]

use crate::Vector;
use crate::ray_tracer::*;
use crate::scene::*;
use crate::Stats;
use crate::RenderSettings;
use crate::matrix::Matrix;
use crate::geometry::*;
use std::f32;
use std::f32::consts;
use crate::math::*;
use rand::Rng;

pub struct LightColorInfo {
    pub brightness: f32,
    pub color: Vector,
    pub exposure: i32
}

impl LightColorInfo {
    pub fn intensity(&self) -> Vector {
        let exp = if self.exposure < 1 {1.0} else {(2.0 as f32).powi(self.exposure)};

        self.color * self.brightness * exp        
    }
}

pub struct LightDistanceInfo {
    pub range: f32,
    pub attenuation: Vector
}

pub struct DirectionalLight {
    pub direction: Vector,
    pub color_info: LightColorInfo
} 

pub struct PointLight {
    pub position: Vector,
    pub color_info: LightColorInfo,
    pub distance_info: LightDistanceInfo
}

pub struct RectangularLight {
    pub position: Vector,
    pub direction: Vector,
    pub rec: Rectangle,
    pub samples: u32,
    pub color_info: LightColorInfo,
    pub distance_info: LightDistanceInfo,
    pub world: Matrix,
    pub s: Vector,
    pub v1: Vector,
    pub v2: Vector,
}

impl DirectionalLight {
    pub fn new(dir: Vector, brightness: f32, color: Vector) -> Self {
        Self {
            direction: dir,
            color_info: LightColorInfo {
                brightness: brightness,
                color: color,
                exposure: 0
            }
        }
    } 

    pub fn intensity(&self) -> Vector {
        self.color_info.intensity()
    }
}

impl PointLight {
    pub fn new(pos: Vector, brightness: f32, color: Vector, range: f32, attenuation: Vector) -> Self {
        Self {
            position: pos,
            color_info: LightColorInfo {
                brightness: brightness,
                color: color,
                exposure: 0
            },
            distance_info: LightDistanceInfo {
                range: range,
                attenuation: attenuation
            }
        }
    }

    pub fn intensity(&self) -> Vector {
        self.color_info.intensity()
    }
}

impl RectangularLight {
    pub fn new(pos: Vector, dir: Vector, width: f32, height: f32, samples: u32, brightness: f32, color: Vector, range: f32, attenuation: Vector) -> Self {
        let up = Vector::vec3(0.0, 1.0, 0.0);

        let look_at = Matrix::look_at_rh(pos, dir, up);
        let world = look_at;

        let s = Vector::vec3(-width * 0.5, -height * 0.5, 0.0) * look_at;
        let v1 = Vector::vec3(width, 0.0, 0.0);
        let v2 = Vector::vec3(0.0, height, 0.0);

        Self {
            position: pos,
            direction: dir,
            rec: Rectangle {
                width: width,
                height: height, 
            },
            samples: samples,
            color_info: LightColorInfo {
                brightness: brightness,
                color: color,
                exposure: 0
            },
            distance_info: LightDistanceInfo {
                range: range,
                attenuation: attenuation
            },
            world: world,
            s: s,
            v1: v1,
            v2: v2
        }
    }

    pub fn intensity(&self) -> Vector {
        self.color_info.intensity() / self.rec.area()
    }
}

pub enum Lights {
    Directional(DirectionalLight),
    Point(PointLight),
    Rectangular(RectangularLight)
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Vector,
    pub specular: Vector,
    pub roughness: f32,
    pub ior: f32,
    pub transmission: f32,
    pub metalicness: f32
}

impl Material {
    pub fn new(albedo: Vector, specular: Vector, roughness: f32, ior: f32, transmission: f32, metalicness: f32) -> Self {
        Self {
            albedo: albedo,
            specular: specular,
            roughness: roughness,
            ior: ior,
            transmission: transmission, 
            metalicness: metalicness
        }
    }
}

pub struct ShadingData {
    position: Vector,
    normal: Vector,
    texture_coord: Vector,
    material: Material
}

impl ShadingData {
    pub fn new (position: Vector, normal: Vector, texture_coord: Vector, material: Material) -> Self {
        Self {
            position: position,
            normal: normal,
            texture_coord: texture_coord,
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
                match trace(data.position + data.normal * 0.0001, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, f32::INFINITY, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                    None => {
                        let v = -dir;
                        let n = data.normal;

                        compute_lighting(data.material.roughness, data.material.specular, n, v, l, 1.0, light.intensity(), &mut diffuse, &mut specular);
                    },
                    _ => ()
                }
            },
            Lights::Point(light) => {
                let mut l = light.position - data.position;
                let distance = l.vec3_length_f32();
                l /= distance;      
                match trace(data.position + data.normal * 0.0001, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, distance, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                    None => {
                        let v = -dir;
                        let n = data.normal;

                        let falloff = 4.0 * consts::PI * distance * distance;
                        compute_lighting(data.material.roughness, data.material.specular, n, v, l, falloff, light.intensity(), &mut diffuse, &mut specular);
                    },
                    _ => ()
                }
            }
            Lights::Rectangular(light) => {
                let mut rec_diffuse = Vector::vec3(0.0, 0.0, 0.0);
                let mut rec_spec = Vector::vec3(0.0, 0.0, 0.0);

                let world = light.world;
                let mut samples = light.samples;

                if ray_type != RayType::CameraRay {
                    samples = 1;
                }

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

                for _ in 0..samples {
                    let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
                    let rand2 = rand::thread_rng().gen_range(0.0, 1.0);

                    let sample_rec = sample_rectangle_uniform(rand1, rand2, &light.rec);
                    let world_pos = sample_rec.0 * world; 

                    let mut l = world_pos - data.position;
                    let distance = l.vec3_length_f32();
                    l /= distance;  

                    let v = -dir;
                    let origin = data.position + data.normal * 0.0001;

                    if intersect_plane(origin, l, light.s, -light.direction.vec3_normalize(), light.v1, light.v2, &mut Vector::vec3(0.0, 0.0, 0.0)) {
                        match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, distance, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                            None => {        
                                let falloff = distance * distance;
    
                                let mut sample_diffuse = Vector::vec3(0.0, 0.0, 0.0);
                                let mut sample_spec = Vector::vec3(0.0, 0.0, 0.0); 
    
                                compute_lighting(data.material.roughness, data.material.specular, n, v, l, falloff, light.intensity(), &mut sample_diffuse, &mut sample_spec);
    
                                rec_diffuse += sample_diffuse / sample_rec.1;     
                            },
                            _ => ()
                        }
                    }

                    let a2 = data.material.roughness * data.material.roughness;
                    let sample = importance_sample_ggx(rand1, rand2, a2);
        
                    let h = (sample.0 * tbn).vec3_normalize();
                    let l = (h * 2.0 * v.vec3_dot(h)) - v;
    
                    let pdf = sample.1;
    
                    let n_o_v = n.vec3_dot_f32(v).abs();
                    let n_o_l = clamp(n.vec3_dot_f32(l), 0.0, 1.0);            
    
                    let mut hit = Vector::vec3(0.0, 0.0, 0.0);
                    if intersect_plane(origin, l, light.s, -light.direction.vec3_normalize(), light.v1, light.v2, &mut hit) {
                        let distance = (data.position - hit).vec3_length_f32();
                        match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, distance, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                            None => {
                                let light_color = light.intensity();

                                let l_o_h = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
                                let n_o_h = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
                    
                                let f = schlick_fresnel_aprx(l_o_h, data.material.specular);
                                let d = ggx_distribution(n_o_h, a2);
                                let g = smith_for_ggx(n_o_l, n_o_v, a2);
                                let res = f * d * g * light_color * n_o_l;
                    
                                rec_spec += res / pdf;
                            },
                            _ => ()
                        }
                    }
                } 

                let a = 1.0 / (samples as f32);
                rec_diffuse *= a;
                rec_spec *= a;

                diffuse += rec_diffuse;
                specular += rec_spec;
            }
        }
    }

    let indirect_light = compute_indirect_light(dir, &data, scene, current_ray_depth, settings, ray_type, stats);
    let color = data.material.albedo / consts::PI * (diffuse + indirect_light.0) + specular + indirect_light.1;
    color.clamp(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(1.0, 1.0, 1.0))
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
    spec_color + (Vector::vec3(1.0, 1.0, 1.0) - spec_color) * (1.0 - l_dot_h).powf(5.0)
}

fn compute_lighting(roughness: f32, specular_color: Vector, n: Vector, v: Vector, l: Vector, falloff: f32, light_intensity: Vector, diffuse: &mut Vector, specular: &mut Vector) {
    let a2 = roughness * roughness;

    let h =  (v + l).vec3_normalize();
    let n_o_v = n.vec3_dot_f32(v).abs();
    let l_o_h = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
    let n_o_h = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
    let n_o_l = clamp(n.vec3_dot_f32(l), 0.0, 1.0);

    let f = schlick_fresnel_aprx(l_o_h, specular_color);
    let d = ggx_distribution(n_o_h, a2);
    let g = smith_for_ggx(n_o_l, n_o_v, a2);
    let brdf = f * g * d;

    let diffuse_term = Vector::vec3(1.0, 1.0, 1.0) - f;
    let energy = (light_intensity / falloff) * n_o_l;
    *specular += brdf * energy;
    *diffuse += diffuse_term * energy;
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

fn compute_indirect_specular(dir: Vector, data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, tbn: &Matrix, specular: & mut Vector, stats: & mut Stats) {

    if settings.specular_samples > 0 && (ray_type == RayType::CameraRay || ray_type == RayType::SpecularRay) {
        let mut samples = settings.specular_samples;

        if current_ray_depth > 0 {
            samples = 1;
        }

        let v = -dir;
        let n = data.normal;
        let a2 = data.material.roughness * data.material.roughness;

        for _ in 0..samples {
            let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
            let rand2 = rand::thread_rng().gen_range(0.0, 1.0);
        
            let sample = importance_sample_ggx(rand1, rand2, a2);
        
            let h = (sample.0 * *tbn).vec3_normalize();
			let l = (h * 2.0 * v.vec3_dot(h)) - v;

            let pdf = sample.1;

            let n_o_v = n.vec3_dot_f32(v).abs();
            let n_o_l = clamp(n.vec3_dot_f32(l), 0.0, 1.0);

            if n_o_v == 0.0 || n_o_l == 0.0 {
                continue;
            }

            let light_color = cast_ray(data.position + l * 0.0001, l, scene, current_ray_depth + 1, settings, RayType::SpecularRay, stats) / pdf;

            let l_o_h = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
            // let v_o_h = clamp(V.vec3_dot_f32(H), 0.0, 1.0);

            let n_o_h = clamp(n.vec3_dot_f32(h), 0.0, 1.0);

            let f = schlick_fresnel_aprx(l_o_h, data.material.specular);
            let d = ggx_distribution(n_o_h, a2);
            let g = smith_for_ggx(n_o_l, n_o_v, a2);
            let res = f * d * g * light_color * n_o_l;

            *specular += res;
        }
    
        *specular /= samples as f32;
    }
}

fn compute_indirect_diffuse(data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, tbn: &Matrix, diffuse: & mut Vector, stats: & mut Stats) {
    if settings.diffuse_samples > 0 && data.material.metalicness < 1.0 {
        let mut samples = settings.diffuse_samples;
        let n = data.normal;

        if current_ray_depth > 0 {
            samples = (settings.diffuse_samples as f32).sqrt() as u32;
        }

        for _ in 0..samples {
            let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
            let rand2 = rand::thread_rng().gen_range(0.0, 1.0);
        
            let sample = sample_hemisphere_cosine_weighted(rand1, rand2);
        
            let dir = (sample.0 * *tbn).vec3_normalize();
            let pdf = sample.1;
            *diffuse += (cast_ray(data.position + dir * 0.0001, dir, scene, current_ray_depth + 1, settings, RayType::DiffuseRay, stats) / pdf) * clamp(dir.vec3_dot_f32(n), 0.0, 1.0);
        }
    
        *diffuse /= samples as f32;
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
	let d = a2 / (consts::PI * d * d);
	let pdf = d * cos_theta;

    (Vector::vec3(x, cos_theta, z), pdf)
}

#[inline]
fn sample_rectangle_uniform(rand1: f32, rand2: f32, rec: &Rectangle) -> (Vector, f32) {
    let x = rand1 * rec.width - rec.width * 0.5;
    let y = rand2 * rec.height - rec.width * 0.5;

    (Vector::vec3(x, y, 0.0), rec.area())
}