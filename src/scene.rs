use crate::geometry::{Mesh, BoundingBox};
use crate::shading::{materials::Material, lights::Lights};
use crate::bvh::LinearBVHNode;
use crate::camera::Camera;

pub struct SceneData {
    pub bvh: Vec<LinearBVHNode>,
    pub object_indices: Vec<usize>,
    pub scene_objects: Vec<SceneObject>,
    pub lights: Vec<Lights>,
    pub camera: Camera
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