use crate::Vector;

pub struct DirectionalLight {
    direction: Vector,
    brightness: f32,
    color: Vector
}

pub struct PointLight {
    position: Vector,
    brightness: f32,
    color: Vector,
    range: f32,
    attenuation: Vector
}

pub enum Lights {
    Directional(DirectionalLight),
    Point(PointLight)
}

pub struct Material {
    pub albedo: Vector
}

impl Material {
    pub fn new(albedo: Vector) -> Self {
        Self {
            albedo: albedo
        }
    }
}

pub struct ShadingData {
    position: Vector,
    normal: Vector,
    textureCoord: Vector
}

impl ShadingData {
    pub fn new (position: Vector, normal: Vector, textureCoord: Vector) -> Self {
        Self {
            position: position,
            normal: normal,
    
            textureCoord: textureCoord
        }
    }
}

pub fn calculate_color(data: ShadingData) -> Vector {
    Vector::vec3(0.7 * 255.0, 0.7 * 255.0, 0.7 * 255.0)
}