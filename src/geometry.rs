#![allow(dead_code)]

use crate::vector::Vector;
use std::f32;

pub trait RayIntersect {
    fn intersect(&self, ray_origin: Vector, ray_dir: Vector) -> Option<f32>;
    fn get_color(&self) -> Vector;
}

pub struct Sphere {
    pub color: Vector,
    pub position: Vector,
    pub radius: f32,
    radius_sqrd: f32
}

impl Sphere {
    pub fn new(color: Vector, position: Vector, radius: f32) -> Self {
        Sphere {
            color: color,
            position: position,
            radius: radius,
            radius_sqrd: radius * radius 
        }
    }
}

impl RayIntersect for Sphere {
    fn intersect(&self, ray_origin: Vector, ray_dir: Vector) -> Option<f32> {
        let L = self.position - ray_origin;

        let tca = L.vec3_dot(ray_dir);

        if tca < 0.0 {
            return None;
        }

        let d2 = L.vec3_dot(L) - tca * tca;

        if d2 > self.radius_sqrd {
            return None;            
        }

        let mut t0;
        let t1;

        let thc = (self.radius_sqrd - d2).sqrt(); 
        t0 = tca - thc; 
        t1 = tca + thc; 

        if t0 < 0.0 { 
            t0 = t1;
            if t0 < 0.0 {
                return None;                 
            } 
        }

        Some(t0) 
    }

    fn get_color(&self) -> Vector {
        self.color
    }
}

pub struct Vertex {
    pub pos: Vector
}

pub struct IntersectResult {
    pub u: f32,
    pub v: f32, 
    pub t: f32
}

pub fn intersect_triangle(ray_origin: Vector, ray_dir: Vector, v_0: Vector, v_1: Vector, v_2:Vector) -> Option<IntersectResult> {
    let v_01 = v_1 - v_0;
    let v_02 = v_2 - v_0;

    let p = ray_dir.vec3_cross(v_02);
    let det = v_01.vec3_dot(p);

    if det.abs() < f32::EPSILON {
        return None;
    }

    let inv_det = 1.0 / det;

    let t_vec = ray_origin - v_0;
    let u = t_vec.vec3_dot(p) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q_vec = t_vec.vec3_cross(v_01);
    let v = ray_dir.vec3_dot(q_vec) * inv_det;

    if v < 0.0 || v + u > 1.0 {
        return None;
    }

    let t = v_02.vec3_dot(q_vec) * inv_det;

    let result = IntersectResult{u: u, v: v, t: t};

    Some(result)
}