#![allow(dead_code)]
pub mod lights;
pub mod materials;
mod brdf;
mod monte_carlo;

use self::materials::Material;
use self::lights::Lights;
use self::brdf::*;
use self::monte_carlo::*;

use crate::{Vector, ray_tracer::*, scene::*, Stats, RenderSettings, matrix::Matrix, geometry::*, math::*};

use std::{f32, f32::consts};
use rand::Rng;

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