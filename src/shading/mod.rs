#![allow(dead_code)]
pub mod lights;
pub mod materials;
mod brdf;
mod monte_carlo;

use self::materials::Material;
use self::lights::*;
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
                    let (light_diffuse, light_spec) = integrate_spherical_light(&light, &data, -dir, scene, current_ray_depth, settings, ray_type, stats) ;
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

                    let l = (world_pos - data.position).vec3_normalize();
                    // let distance = l.vec3_length_f32();
                    // l /= distance;  

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
                    let l = ((h * 2.0 * v.vec3_dot(h)) - v).vec3_normalize();       
    
                    hit = Vector::vec3(0.0, 0.0, 0.0);
                    if intersect_plane(origin, l, light.s, -light.direction.vec3_normalize(), light.v1, light.v2, &mut hit) {
                        let distance = (data.position - hit).vec3_length_f32();
                        match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, distance, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                            None => {
                                let falloff = distance * distance;

                                let light_color = light.intensity() / falloff;

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

fn integrate_spherical_light(light: &SphericalLight, data: &ShadingData, view: Vector, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, stats: & mut Stats) -> (Vector, Vector) {
    let mut light_diffuse = Vector::vec3(0.0, 0.0, 0.0);
    let mut light_spec = Vector::vec3(0.0, 0.0, 0.0);

    let mut samples = light.samples;
    let sampling_strats = 1;

    let n =  data.normal;

    if ray_type != RayType::CameraRay {
        samples = 1;
    }
    
    //create coords
    let mut w = light.position - data.position;
    let distance = w.vec3_length_f32();
    w /= distance;
    let (v, u) = create_orthonormal_coordinate_system(w);

    let to_world = Matrix::from_vector(
        v, w, u, Vector::vec4(0.0, 0.0, 0.0, 1.0)
    );

    let r = light.radius / distance;
    let q = (1.0 - r * r).sqrt();

    //tbn 
    // let (t, b) = create_orthonormal_coordinate_system(n);
    
    // let tbn = Matrix::from_vector(
    //     t, n, b, Vector::vec4(0.0, 0.0, 0.0, 1.0)
    // );

    for _ in 0..samples {
        let rand1 = rand::thread_rng().gen_range(0.0, 1.0);
        let rand2 = rand::thread_rng().gen_range(0.0, 1.0);
    
        //sample diffuse and specular via solid angle
        let l = (sample_solid_angle_of_sphere(rand1, rand2, q) * to_world).vec3_normalize();
        let origin = data.position + l * 0.0001;
        let hit = &mut Vector::vec3(0.0, 0.0, 0.0);

        if intersect_sphere(light.position, light.radius * light.radius, origin, l, hit) {
            let dist = (*hit - data.position).vec3_length_f32();

            match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, dist, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
                None => {        
                    let falloff = dist * dist;

                    let h =  (view + l).vec3_normalize();
                    let dot_nh = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
                    let dot_nl = clamp(n.vec3_dot_f32(l), 0.0, 1.0);
                    let dot_nv = clamp(n.vec3_dot_f32(view), 0.0, 1.0);
                    let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, 1.0);

                    let sample_diffuse = compute_diffuse(dot_nv, dot_nl, dot_nh, data.material.roughness, falloff, light.intensity());
                    let sample_spec = compute_specular(n, l, view, data.material.specular, dot_lh, dot_nl, dot_nh, data.material.roughness, falloff, light.intensity());

                    let pdf = 1.0 / (consts::PI * (1.0 - q));
                    
                    light_diffuse += sample_diffuse / pdf; 
                    light_spec += sample_spec / pdf;
                },
                _ => ()
            }
        }

        //sample brdf - specular
        // let a2 = data.material.roughness * data.material.roughness;
        // let (sample, pdf) = importance_sample_ggx(rand1, rand2, a2);

        // let h = (sample * tbn).vec3_normalize();
        // let l = ((h * 2.0 * view.vec3_dot(h)) - view).vec3_normalize();
        // let origin = data.position + l * 0.0001;
        // let hit = &mut Vector::vec3(0.0, 0.0, 0.0);

        // let dot_nv = clamp(n.vec3_dot_f32(view), 0.0, 1.0);
        // let dot_nl = clamp(n.vec3_dot_f32(l), 0.0, 1.0);

        // if dot_nv == 0.0 || dot_nl == 0.0 {
        //     continue;
        // }

        // if intersect_sphere(light.position, light.radius * light.radius, origin, l, hit) {
        //     let dist = (*hit - data.position).vec3_length_f32();

        //     match trace(origin, l, &scene.scene_objects, &scene.bvh, &scene.object_indices, dist, current_ray_depth + 1, settings, RayType::ShadowRay, stats) {
        //         None => {        
        //             let falloff = 1.0 / (dist * dist);

        //             let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, l.vec3_dot_f32(h));
        //             let f = schlick_fresnel_aprx(dot_lh, data.material.specular);
        //             let d = ggx_distribution(n.vec3_dot_f32(h), a2);
        //             let g = height_correlated_smith_shadow_and_masking_for_ggx(n, l, view, a2);
        
        //             let pdf = (d * n.vec3_dot_f32(h)) / (4.0 * view.vec3_dot_f32(h).abs());
        //             let brdf = (f * d * g * n.vec3_dot_f32(l).abs()) / (4.0 * n.vec3_dot_f32(l).abs() * n.vec3_dot_f32(view).abs());
        //             let reflectance = (brdf / pdf) * falloff * light.intensity();
                  
        //             // let weight = ((view.vec3_dot_f32(h).abs())) / (n.vec3_dot_f32(view).abs() * n.vec3_dot_f32(h).abs());
        //             // let reflectance = f * g * weight * falloff * light.intensity();
            
        //             // light_spec += reflectance;
        //         },
        //         _ => ()
        //     }
        // }
    }
    
    light_diffuse *= 1.0 / samples as f32;
    light_spec *= 1.0 / (samples * sampling_strats) as f32;

    (light_diffuse, light_spec)
}

fn compute_diffuse(dot_nv: f32, dot_nl: f32, dot_nh: f32, roughness: f32, falloff: f32, light_intensity: Vector) -> Vector {
    let energy = (light_intensity / falloff) * dot_nl;
    energy * disney_diffuse_model(dot_nv, dot_nl, dot_nh, roughness)
}

fn compute_specular(n: Vector, l: Vector, v: Vector, specular_color: Vector, dot_lh: f32, dot_nl: f32, dot_nh: f32, roughness: f32, falloff: f32, light_intensity: Vector) -> Vector {
    let a2 = roughness * roughness;
    let energy = (light_intensity / falloff) * dot_nl;
    let f = schlick_fresnel_aprx(dot_lh, specular_color);
    let d = ggx_distribution(dot_nh, a2);
    let g = height_correlated_smith_shadow_and_masking_for_ggx(n, l, v, a2);

    let brdf = (f * d * g) / (4.0 * n.vec3_dot_f32(l).abs() * n.vec3_dot_f32(v).abs());
    brdf * energy
}

fn compute_lighting(roughness: f32, specular_color: Vector, n: Vector, v: Vector, l: Vector, falloff: f32, light_intensity: Vector, diffuse: &mut Vector, specular: &mut Vector) {
    let a2 = roughness * roughness;

    let h =  (v + l).vec3_normalize();
    let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
    let dot_nh = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
    let dot_nl = clamp(n.vec3_dot_f32(l), 0.0, 1.0);
    // let ga = (0.5 + roughness / 2.0).powi(2);

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

            // let dot_nv = n.vec3_dot_f32(v).abs();
            let dot_nv = clamp(n.vec3_dot_f32(v), 0.0, 1.0);
            let dot_nl = clamp(n.vec3_dot_f32(l), 0.0, 1.0);

            if dot_nv == 0.0 || dot_nl == 0.0 {
                continue;
            }

            let light_color = cast_ray(data.position + l * 0.0001, l, scene, current_ray_depth + 1, settings, RayType::SpecularRay, stats);

            let dot_lh = clamp(l.vec3_dot_f32(h), 0.0, 1.0);
            let dot_nh = clamp(n.vec3_dot_f32(h), 0.0, 1.0);
            // let dot_vh = clamp(v.vec3_dot_f32(h), 0.0, 1.0);

            // let ga = (0.5 + data.material.roughness / 2.0).powi(2);
            let f = schlick_fresnel_aprx(dot_lh, data.material.specular);
            let d = ggx_distribution(n.vec3_dot_f32(h), a2);
            let g = height_correlated_smith_shadow_and_masking_for_ggx(n, l, v, a2);

            let pdf = (d * n.vec3_dot_f32(h)) / (4.0 * v.vec3_dot_f32(h).abs());
            let brdf = (f * d * g * n.vec3_dot_f32(l).abs()) / (4.0 * n.vec3_dot_f32(l).abs() * n.vec3_dot_f32(v).abs());
            let reflectance = (brdf / pdf) * light_color;

            // let weight = ((v.vec3_dot_f32(h).abs())) / (n.vec3_dot_f32(v).abs() * n.vec3_dot_f32(h).abs());
            // let reflectance = f * g * weight * light_color;

            *specular += reflectance;
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