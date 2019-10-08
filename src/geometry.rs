#![allow(dead_code)]

use crate::vector::Vector;

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