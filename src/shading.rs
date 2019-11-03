use crate::Vector;

pub struct DirectionalLight {
    pub direction: Vector,
    pub brightness: f32,
    pub color: Vector
} 

pub struct PointLight {
    pub position: Vector,
    pub brightness: f32,
    pub color: Vector,
    pub range: f32,
    pub attenuation: Vector
}

impl DirectionalLight {
    pub fn new(dir: Vector, brightness: f32, color: Vector) -> Self {
        Self {
            direction: dir,
            brightness: brightness,
            color: color
        }
    } 
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

pub fn calculate_color(data: ShadingData, lights: &[Lights]) -> Vector {
    let mut diffuse = Vector::vec3(0.0, 0.0, 0.0);

    for i in 0..lights.len() {
        match &lights[i] {
            Lights::Directional(d) => {
                let dir_normalized = d.direction.vec3_normalize() * - 1.0;
                diffuse += d.color * dir_normalized.vec3_dot(data.normal.vec3_normalize()).max(0.0) * d.brightness;
            },
            Lights::Point(point) => ()
        }
    }

    Vector::vec3(0.7, 0.7, 0.7) * diffuse
    // Vector::vec3(data.normal.x() + 1.0, data.normal.y() + 1.0, data.normal.z() + 1.0) * 0.5
}