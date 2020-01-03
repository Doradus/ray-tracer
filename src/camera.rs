use crate::Vector;
use crate::matrix::Matrix;

struct Perspective {
    fov: f32
}

struct Orthographic {
    ortho_width: f32
}

enum ProjectionType {
    Perspective(Perspective),
    Orthographic(Orthographic)
}

pub struct Camera {
    pub position: Vector,
    pub target: Vector,
    pub to_world: Matrix
}

impl Camera {
    pub fn new(position: Vector, target: Vector) -> Self {
        let up = Vector::vec3(0.0, 1.0, 0.0);
        let camera_to_world = Matrix::look_at_rh(position, target, up);

        Self {
            position: position,
            target: target,
            to_world: camera_to_world
        }
    }
}