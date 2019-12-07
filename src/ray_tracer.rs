use crate::vector::Vector;
use crate::geometry::{Mesh, BoundingBox};
use crate::scene::*;
use crate::shading::{calculate_color, ShadingData};
use crate::Stats;
use crate::RenderSettings;
use std::f32;

#[derive(PartialEq, Copy, Clone)]
pub enum RayType {
    CameraRay,
    ShadowRay,
    SpecularRay,
    DiffuseRay
} 

pub fn cast_ray(origin: Vector, direction: Vector, scene: &SceneData, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, stats: & mut Stats) -> Vector {
    match trace(origin, direction, &scene.scene_objects, f32::INFINITY, current_ray_depth, settings, ray_type, stats) {
        None => Vector::vec3(0.0, 0.0, 0.0),
        Some(i) => {
            let mesh = &scene.scene_objects[i.mesh_index].mesh;
            
            let ind_1 = mesh.indices[i.triangle_index] as usize;
            let ind_2 = mesh.indices[i.triangle_index + 1] as usize;
            let ind_3 = mesh.indices[i.triangle_index + 2] as usize;

            let v_0 = &mesh.vertices[ind_1];
            let v_1 = &mesh.vertices[ind_2];
            let v_2 = &mesh.vertices[ind_3];

            let position = origin + direction * i.t;
            
            let normal = (v_0.normal.vec3_normalize() * (1.0 - i.u - i.v) + v_1.normal.vec3_normalize() * i.u + v_2.normal.vec3_normalize() * i.v).vec3_normalize();
            let data = ShadingData::new(position, normal, Vector::vec2(0.0, 0.0), scene.scene_objects[i.mesh_index].material);

            calculate_color(data, direction, scene, current_ray_depth, settings, ray_type, stats)
        }
    }
}

pub struct TraceResult {
    u: f32,
    v: f32,
    triangle_index: usize,
    mesh_index: usize,
    t: f32
}

pub fn trace(origin: Vector, direction: Vector, scene_objects: &[SceneObject], near: f32, current_ray_depth: u32, settings: RenderSettings, ray_type: RayType, stats: & mut Stats) -> Option<TraceResult> {
    let mut found:Option<TraceResult> = None;
    
    if current_ray_depth > settings.max_ray_depth {
        return found;
    }
    
    stats.num_rays_shot += 1;
    let mut closest = near;

    let inv_dir = Vector::vec3(1.0 / direction.x(), 1.0 / direction.y(), 1.0 / direction.z());
    let sign_x = if inv_dir.x() < 0.0 {1.0} else {0.0};
    let sign_y = if inv_dir.y() < 0.0 {1.0} else {0.0};
    let sign_z = if inv_dir.z() < 0.0 {1.0} else {0.0};
    let sign = Vector::vec3(sign_x, sign_y, sign_z);

    for i in 0..scene_objects.len() {
        if scene_objects[i].bounding_box.intersect(origin, inv_dir, sign) {
            match intersect_mesh(origin, direction, &scene_objects[i].mesh, stats) {
                Some(mesh_result) => {
                    if mesh_result.t < closest {
                        closest = mesh_result.t;

                        let result = TraceResult {
                            u: mesh_result.u,
                            v: mesh_result.v,
                            triangle_index: mesh_result.triangle_index,
                            mesh_index: i,
                            t: mesh_result.t
                        };
                        found = Some(result);
                    }
                },
                None => ()
            }
        }
    }

    found
}

struct MeshIntersectResult {
    u: f32,
    v: f32,
    triangle_index: usize,
    t: f32
}

fn intersect_mesh(origin: Vector, direction: Vector, scene_object: &Mesh, stats: & mut Stats) -> Option<MeshIntersectResult> {
        let mut found:Option<MeshIntersectResult> = None;
        let mut closest = f32::INFINITY;

       let mut index = 0;
       for _ in 0..scene_object.num_tris {
            let v_0 = scene_object.indices[index] as usize;
            let v_1 = scene_object.indices[index + 1] as usize;
            let v_2 = scene_object.indices[index + 2] as usize;

            match intersect_triangle(origin, direction, scene_object.vertices[v_0].pos, scene_object.vertices[v_1].pos, scene_object.vertices[v_2].pos, stats) {
                Some(tri_result) => {
                    if tri_result.t < closest {
                        closest = tri_result.t;

                        let result = MeshIntersectResult {
                            u: tri_result.u,
                            v: tri_result.v,
                            triangle_index: index,
                            t: closest
                        };
                        found = Some(result);
                    }
                },
                None => ()
            }

            index += 3;
        }

    found    
}

struct TriangleIntersectResult {
    pub u: f32,
    pub v: f32, 
    pub t: f32
}

fn intersect_triangle(ray_origin: Vector, ray_dir: Vector, v_0: Vector, v_1: Vector, v_2:Vector, stats: & mut Stats) -> Option<TriangleIntersectResult> {
    stats.num_tringle_tests += 1;

    let v0v1  = v_1 - v_0;
    let v0v2  = v_2 - v_0;

    let p = ray_dir.vec3_cross(v0v2);
    let det = v0v1.vec3_dot(p);

    if det < f32::EPSILON {
        return None;
    }

    let inv_det = 1.0 / det;

    let t_vec = ray_origin - v_0;
    let u = t_vec.vec3_dot(p) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q_vec = t_vec.vec3_cross(v0v1);
    let v = ray_dir.vec3_dot(q_vec) * inv_det;

    if v < 0.0 || v + u > 1.0 {
        return None;
    }

    let t = v0v2.vec3_dot(q_vec) * inv_det;

    if t < 0.0 {
        return None;
    }

    let result = TriangleIntersectResult{u: u, v: v, t: t};

    stats.num_triangles_intersected += 1;

    Some(result)
}