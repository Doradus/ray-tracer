use crate::vector::Vector;
use crate::geometry::{Mesh};
use crate::scene::*;
use std::f32;

pub fn cast_ray(origin: Vector, direction: Vector, scene: &SceneData) -> Vector {
    match trace(origin, direction, &scene.scene_objects) {
        None => Vector::vec3(0.0, 0.0, 0.0),
        Some(i) => {
            let ind_1 = scene.scene_objects[i.mesh_index].mesh.indices[i.triangle_index] as usize;
            let ind_2 = scene.scene_objects[i.mesh_index].mesh.indices[i.triangle_index + 1] as usize;
            let ind_3 = scene.scene_objects[i.mesh_index].mesh.indices[i.triangle_index + 2] as usize;

            let v_1 = (scene.scene_objects[i.mesh_index].mesh.vertices[ind_1].normal + 1.0) * 0.5 * 255.0;
            let v_2 = (scene.scene_objects[i.mesh_index].mesh.vertices[ind_2].normal + 1.0) * 0.5 * 255.0;
            let v_3 = (scene.scene_objects[i.mesh_index].mesh.vertices[ind_3].normal + 1.0) * 0.5 * 255.0;

            let color = v_1 * (1.0 - i.u - i.v) + v_2 * i.u + v_3 * i.v;
            // let color = Vector::vec3((1.0 - i.u - i.v) * 255.0, i.u * 255.0, i.v * 255.0);
            color
        }
    }
}

struct TraceResult {
    u: f32,
    v: f32,
    triangle_index: usize,
    mesh_index: usize
}

//will find the nearest object
fn trace(origin: Vector, direction: Vector, scene_objects: &[SceneObject]) -> Option<TraceResult> {
    let mut found:Option<TraceResult> = None;
    let mut closest = f32::INFINITY;

    for i in 0..scene_objects.len() {
        match intersect_mesh(origin, direction, &scene_objects[i].mesh) {
            Some(mesh_result) => {
                if mesh_result.t < closest {
                    closest = mesh_result.t;

                    let result = TraceResult {
                        u: mesh_result.u,
                        v: mesh_result.v,
                        triangle_index: mesh_result.triangle_index,
                        mesh_index: i
                    };
                    found = Some(result);
                }
            },
            None => ()
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

fn intersect_mesh(origin: Vector, direction: Vector, scene_object: &Mesh) -> Option<MeshIntersectResult> {
        let mut found:Option<MeshIntersectResult> = None;
        let mut closest = f32::INFINITY;

       let mut index = 0;
       for _ in 0..scene_object.num_tris {
            let v_0 = scene_object.indices[index] as usize;
            let v_1 = scene_object.indices[index + 1] as usize;
            let v_2 = scene_object.indices[index + 2] as usize;

            match intersect_triangle(origin, direction, scene_object.vertices[v_0].pos, scene_object.vertices[v_1].pos, scene_object.vertices[v_2].pos) {
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

fn intersect_triangle(ray_origin: Vector, ray_dir: Vector, v_0: Vector, v_1: Vector, v_2:Vector) -> Option<TriangleIntersectResult> {
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

    let result = TriangleIntersectResult{u: u, v: v, t: t};

    Some(result)
}