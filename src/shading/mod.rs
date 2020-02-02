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
            Lights::Spherical(light) => {
                if light.radius == 0.0 {
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
                } else {
                    let beta = 1;
                    let mut light_diffuse = Vector::vec3(0.0, 0.0, 0.0);
                    let mut light_spec = Vector::vec3(0.0, 0.0, 0.0);
                    
                    //create coords
                    let mut w = light.position - data.position;
                    let distance = w.vec3_length_f32();
                    w /= distance;

                    let (v, u) = create_orthonormal_coordinate_system(w);

                    let to_world = Matrix::from_vector(
                        v, w, u, Vector::vec4(0.0, 0.0, 0.0, 1.0)
                    );

                    let mut samples = light.samples;

                    let n = data.normal;
                    let (t, b) = create_orthonormal_coordinate_system(n);
                
                    let tbn = Matrix::from_vector(
                        t, n, b, Vector::vec4(0.0, 0.0, 0.0, 1.0)
                    );

                    if ray_type != RayType::CameraRay {
                        samples = 1;
                    }

                    let x = light.radius / distance;
                    let r = (1.0 - x * x).sqrt();

                    let a2 = data.material.roughness * data.material.roughness;

                    for _ in 0..samples {
                        let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
                        let rand2 = rand::thread_rng().gen_range(0.0, 1.0);

                        let phi = 2.0 * consts::PI * rand1;
                        let theta = (1.0 - rand2 + rand2 * r).acos();

                        let cos_theta = theta.cos();
                        let sin_theta = theta.sin();

                        let x = sin_theta * phi.cos();
                        let z = sin_theta * phi.sin();
                    
                        let l = Vector::vec3(x, cos_theta, z) * to_world;
                        let origin = data.position + data.normal * 0.0001;
                        let hit = &mut Vector::vec3(0.0, 0.0, 0.0);

                        if intersect_sphere(light.position, light.radius * light.radius, origin, l.vec3_normalize(), hit) {
                            let dist = (*hit - data.position).vec3_length_f32();

                            match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, dist, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                                None => {        
                                    let falloff = dist * dist;
        
                                    let mut sample_diffuse = Vector::vec3(0.0, 0.0, 0.0);
                                    let mut sample_spec = Vector::vec3(0.0, 0.0, 0.0); 
        
                                    compute_lighting(data.material.roughness, data.material.specular, data.normal, -dir, l, falloff, light.intensity(), &mut sample_diffuse, &mut sample_spec);
        
                                    let pdf = 1.0 / (consts::PI * (1.0 - r));
                                    let d = (cos_theta * a2 - cos_theta) * cos_theta + 1.0;
                                    let d = a2 / (consts::PI * d * d);
                                    let ggx_pdf = d * cos_theta;
                                    
                                    let combined_pdf = pdf.powi(beta) + ggx_pdf.powi(beta);

                                    // light_diffuse += sample_diffuse / pdf; 
                                    // light_spec += sample_spec / pdf;     

                                    light_spec += (sample_spec * pdf.powi(beta)) / (combined_pdf * pdf);
                                },
                                _ => ()
                            }
                        }

                        let (sample, pdf) = importance_sample_ggx(rand1, rand2, a2);
            
                        let spec_v = -dir;
                        let h = (sample * tbn).vec3_normalize();
                        let l_spec = (h * 2.0 * spec_v.vec3_dot(h)) - spec_v;
        
                        let n_o_v = n.vec3_dot_f32(spec_v).abs();
                        let n_o_l = clamp(n.vec3_dot_f32(l_spec), 0.0, 1.0);            
        
                        let spec_hit = &mut Vector::vec3(0.0, 0.0, 0.0);
                        if intersect_sphere(light.position, light.radius * light.radius, origin, l_spec.vec3_normalize(), spec_hit) {
                            let dist = (*spec_hit - data.position).vec3_length_f32();

                            match trace(origin, l_spec, &scene.scene_objects, &scene.bvh, &scene.object_indices, dist, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                                None => {
                                    let light_color = light.intensity() * (1.0 / (dist * dist));
    
                                    let l_o_h = clamp(l_spec.vec3_dot_f32(h), 0.0, 1.0);
                                    let n_o_h = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
                        
                                    let f = schlick_fresnel_aprx(l_o_h, data.material.specular);
                                    let d = ggx_distribution(n_o_h, a2);
                                    let g = smith_for_ggx(n_o_l, n_o_v, a2);
                                    let sample_spec = f * d * g * light_color * n_o_l;
                        
                                    let solid_angle_pdf = 1.0 / (consts::PI * (1.0 - r));
                                    let combined_pdf = pdf.powi(beta) + solid_angle_pdf.powi(beta);

                                    // light_spec += sample_spec / pdf;   
                                    light_spec += (sample_spec * pdf.powi(beta)) / (combined_pdf * pdf);
                                },
                                _ => ()
                            }
                        }
                    }

                    light_diffuse *= 1.0 / (samples as f32);
                    light_spec *= 1.0 / (samples as f32 * 2.0);

                    diffuse += light_diffuse;
                    specular += light_spec;
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

                let n = data.normal;
                let (t, b) = create_orthonormal_coordinate_system(n);
            
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
                    let mut hit = Vector::vec3(0.0, 0.0, 0.0);

                    if intersect_plane(origin, l, light.s, -light.direction.vec3_normalize(), light.v1, light.v2, &mut hit) {
                        let distance = (data.position - hit).vec3_length_f32();

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
    
                    hit = Vector::vec3(0.0, 0.0, 0.0);
                    if intersect_plane(origin, l, light.s, -light.direction.vec3_normalize(), light.v1, light.v2, &mut hit) {
                        let distance = (data.position - hit).vec3_length_f32();
                        match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, distance, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                            None => {
                                let light_color = light.intensity();

                                let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
                                let n_o_h = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
                    
                                let f = schlick_fresnel_aprx(dot_lh, data.material.specular);
                                let g = height_correlated_smith_shadow_and_masking_for_ggx(n, l, v, a2);
          
                                let weight = (f * g * (v.vec3_dot_f32(h).abs())) / (n.vec3_dot_f32(v).abs() * n.vec3_dot_f32(h).abs());
                                let res = weight * light_color;
                    
                                rec_spec += res;
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

    let (indirect_diffuse, indirect_spec) = compute_indirect_light(dir, &data, scene, current_ray_depth, settings, ray_type, stats);
    let color = (data.material.albedo / consts::PI) * (diffuse + indirect_diffuse) + specular + indirect_spec;
    color
    // color.clamp(Vector::vec3(0.0, 0.0, 0.0), Vector::vec3(1.0, 1.0, 1.0))
}

fn compute_lighting(roughness: f32, specular_color: Vector, n: Vector, v: Vector, l: Vector, falloff: f32, light_intensity: Vector, diffuse: &mut Vector, specular: &mut Vector) {
    let a2 = roughness * roughness;

    let h =  (v + l).vec3_normalize();
    let dot_nv = n.vec3_dot_f32(v).abs();
    let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
    let dot_nh = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
    let dot_nl = clamp(n.vec3_dot_f32(l), 0.0, 1.0);
    let ga = (0.5 + roughness / 2.0).powi(2);

    let f = schlick_fresnel_aprx(dot_lh, specular_color);
    let d = ggx_distribution(dot_nh, a2);
    let g = height_correlated_smith_shadow_and_masking_for_ggx(n, l, v, a2);
    let brdf = (f * d * g) / (4.0 * n.vec3_dot_f32(l).abs() * n.vec3_dot_f32(v).abs());

    let diffuse_term = Vector::vec3(1.0, 1.0, 1.0) - f;
    let energy = (light_intensity / falloff) * dot_nl;
    *specular += brdf * energy;
    *diffuse += diffuse_term * energy;
}

fn compute_indirect_light(dir: Vector, data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, stats: & mut Stats) -> (Vector, Vector) {
    let mut indirect_diffuse = Vector::vec3(0.0, 0.0, 0.0);
    let mut indirect_specular = Vector::vec3(0.0, 0.0, 0.0);

    if current_ray_depth < settings.max_ray_depth {
        let n = data.normal;

        let (t, b) = create_orthonormal_coordinate_system(n);
    
        let tbn = Matrix::from_vector(
            t, n, b, Vector::vec4(0.0, 0.0, 0.0, 1.0)
        );

        compute_indirect_diffuse(data, scene, current_ray_depth, settings, ray_type, &tbn, &mut indirect_diffuse, stats);
        compute_indirect_specular(dir, data, scene, current_ray_depth, settings, ray_type, &tbn, &mut indirect_specular, stats);
    }

    (indirect_diffuse, indirect_specular)
}

fn compute_indirect_specular(dir: Vector, data: &ShadingData, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, tbn: &Matrix, specular: & mut Vector, stats: & mut Stats) {

    if settings.specular_samples > 0 {
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
        
            let (sample, pdf) = importance_sample_ggx(rand1, rand2, a2);
        
            let h = (sample * *tbn).vec3_normalize();
			let l = ((h * 2.0 * v.vec3_dot(h)) - v).vec3_normalize();

            let dot_nv = n.vec3_dot_f32(v).abs();
            // let dot_nv = clamp(n.vec3_dot_f32(v), 0.0, 1.0);
            let dot_nl = clamp(n.vec3_dot_f32(l), 0.0, 1.0);

            if dot_nv == 0.0 || dot_nl == 0.0 {
                continue;
            }

            let light_color = cast_ray(data.position + l * 0.0001, l, scene, current_ray_depth + 1, settings, RayType::SpecularRay, stats);

            let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, l.vec3_dot_f32(h));
            let dot_nh = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
            let dot_vh = clamp(v.vec3_dot_f32(h), 0.0, 1.0);

            let ga = (0.5 + data.material.roughness / 2.0).powi(2);
            let f = schlick_fresnel_aprx(dot_lh, data.material.specular);
            // let d = ggx_distribution(dot_nh, a2);
            let g = height_correlated_smith_shadow_and_masking_for_ggx(n, l, v, a2);

            // let pdf_spec = (dot_nh) / (4.0 * dot_lh);

            // let weight = f * g * d;
            let weight = (f * g * (v.vec3_dot_f32(h).abs())) / (n.vec3_dot_f32(v).abs() * n.vec3_dot_f32(h).abs());
            let res = weight * light_color;

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
        
            let (sample, pdf) = sample_hemisphere_cosine_weighted(rand1, rand2);
        
            let dir = (sample * *tbn).vec3_normalize();

            *diffuse += (cast_ray(data.position + dir * 0.0001, dir, scene, current_ray_depth + 1, settings, RayType::DiffuseRay, stats) / pdf) * clamp(dir.vec3_dot_f32(n), 0.0, 1.0);
        }
    
        *diffuse /= samples as f32;
    }
}