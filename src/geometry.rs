#![allow(dead_code)]

// mod vec3;
// use vec3::Vec3;
use crate::vec3::Vec3;

pub trait RayIntersect {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<f32>;
    fn get_color(&self) -> Vec3;
}

pub struct Sphere {
    pub color: Vec3,
    pub position: Vec3,
    pub radius: f32,
    radius_sqrd: f32
}

impl Sphere {
    pub fn new(color: Vec3, position: Vec3, radius: f32) -> Self {
        Sphere {
            color: color,
            position: position,
            radius: radius,
            radius_sqrd: radius * radius 
        }
    }
}

impl RayIntersect for Sphere {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<f32> {
        let L = self.position - ray_origin;
        
        let tca = L.dot(ray_dir);

        if tca < 0.0 {
            return None;
        }

        let d2 = L.dot(L) - tca * tca;

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

    fn get_color(&self) -> Vec3 {
        self.color
    }
}