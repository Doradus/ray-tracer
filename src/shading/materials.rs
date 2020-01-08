use crate::Vector;

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