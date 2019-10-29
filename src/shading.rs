use crate::Vector;
use crate::geometry::Vertex;

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

// pub fn calculate_color(material: &Material, verts: &[Vertex], t_0: f32, t_1: f32) -> Vector {

// }