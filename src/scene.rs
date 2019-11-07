use crate::geometry::Mesh;
use crate::shading::{Material, Lights};
use crate::Vector;
use std::fmt;

pub struct SceneData {
    pub scene_objects: Vec<SceneObject>,
    pub lights: Vec<Lights>
}

pub struct SceneObject {
    pub mesh: Mesh,
    pub material: Material,
    pub bounding_box: BoundingBox
}

pub struct BoundingBox {
    pub bounds:Vec<Vector>
}



impl BoundingBox {
    pub fn new(pos: Vector) -> Self {
        Self {
            bounds: vec![Vector::vec3(pos.x(), pos.y(), pos.z()), Vector::vec3(pos.x(), pos.y(), pos.z())]
        }
    }
    
    pub fn extend_bounds(&mut self, pos: Vector) {
        if pos.x() < self.bounds[0].x() {self.bounds[0].set_x(pos.x())};
        if pos.y() < self.bounds[0].y() {self.bounds[0].set_y(pos.y())};
        if pos.z() < self.bounds[0].z() {self.bounds[0].set_z(pos.z())};

        if pos.x() > self.bounds[1].x() {self.bounds[1].set_x(pos.x())};
        if pos.y() > self.bounds[1].y() {self.bounds[1].set_y(pos.y())};
        if pos.z() > self.bounds[1].z() {self.bounds[1].set_z(pos.z())};
    }

    pub fn intersect(&self, ray_origin: Vector, inv_ray_dir: Vector, sign: Vector) -> bool {
        let sign_x = sign.x() as usize;
        let sign_y = sign.y() as usize;
        let sign_z = sign.z() as usize;

        let mut t_min = (self.bounds[sign_x].x() - ray_origin.x()) * inv_ray_dir.x();
        let mut t_max = (self.bounds[1 - sign_x].x() - ray_origin.x()) * inv_ray_dir.x();

        let t_y_min = (self.bounds[sign_y].y() - ray_origin.y()) * inv_ray_dir.y();
        let t_y_max = (self.bounds[1 - sign_y].y() - ray_origin.y()) * inv_ray_dir.y();

        if (t_min > t_y_max) || (t_y_min > t_max) {
            return false;
        }

        if t_y_min > t_min {
            t_min = t_y_min;
        }

        if t_y_max < t_max {
            t_max = t_y_max;
        }

        let t_z_min = (self.bounds[sign_z].z() - ray_origin.z()) * inv_ray_dir.z();
        let t_z_max = (self.bounds[1 - sign_z].z() - ray_origin.z()) * inv_ray_dir.z();

        if (t_min > t_z_max) || (t_z_min > t_max) {
            return false;
        }

        true
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "min bounds: {},\n max bounds: {}", self.bounds[0], self.bounds[1]
        )
    }
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