use crate::geometry::Mesh;
use crate::shading::{Material, Lights};

pub struct SceneData {
    pub scene_objects: Vec<SceneObject>,
    pub lights: Vec<Lights>
}

pub struct SceneObject {
    pub mesh: Mesh,
    pub material: Material,
}

impl SceneObject {
    pub fn new(mesh: Mesh, material: Material) -> Self {
        Self {
            mesh: mesh,
            material: material
        }
    }
}