use std::f32::consts;
use crate::Vector;

#[inline]
pub fn clamp<T>(value: T, min: T, max: T) -> T 
    where T : PartialOrd {
        if value < min {
            return min;
        } else if value > max {
            return max;
        } else {
            value
        }
}

#[inline]
pub fn degree_to_radians(deg: f32) -> f32 {
    deg * consts::PI / 180.0 
}

#[inline]
pub fn create_orthonormal_coordinate_system(v1: Vector) -> (Vector, Vector) {
    let v2;
    let v3;

    if v1.x().abs() > v1.y().abs() {
        v2 = Vector::vec3(v1.z(), 0.0, -v1.x()) / (v1.x() * v1.x() + v1.z() * v1.z()).sqrt();
    } else {
        v2 = Vector::vec3(0.0, -v1.z(), v1.y()) / (v1.y() * v1.y() + v1.z() * v1.z()).sqrt();
    }

    v3 = v1.vec3_cross(v2);

    (v2, v3)
}