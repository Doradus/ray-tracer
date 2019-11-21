use crate::geometry::{Mesh, BoundingBox};
use crate::shading::{Material, Lights};


pub struct SceneData {
    pub scene_objects: Vec<SceneObject>,
    pub lights: Vec<Lights>
}

pub struct SceneObject {
    pub mesh: Mesh,
    pub material: Material,
    pub bounding_box: BoundingBox
}

impl SceneObject {
    pub fn new(mesh: Mesh, material: Material, bounding_box: BoundingBox) -> Self {
        Self {
            mesh: mesh,
            material: material,
            bounding_box: bounding_box
        }
    }
}