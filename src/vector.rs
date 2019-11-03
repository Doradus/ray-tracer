#![allow(dead_code)]
use std::{ops, fmt};
use crate::matrix::Matrix;

#[derive(Clone, Copy, Debug)]
pub struct Vector (f32, f32, f32, f32);


impl Vector {

    #[inline]
    pub fn vec2(x: f32, y: f32) -> Self {
        Self (x, y, 0.0, 0.0)
    }

    #[inline]
    pub fn vec3(x: f32, y: f32, z: f32) -> Self {
        Self (x, y, z, 0.0)
    }

    #[inline]
    pub fn vec3_dot(self, v2: Self) -> f32 {
        self.0 * v2.0 + self.1 * v2.1 + self.2 * v2.2
    }

    #[inline]
    pub fn vec3_length(self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    #[inline]
    pub fn vec3_length_reciprocal(self) -> f32 {
        1.0 / self.vec3_length()
    }

    #[inline]
    pub fn vec3_normalize(self) -> Self {
        self * self.vec3_length_reciprocal()
    }

    #[inline]
    pub fn vec3_cross(self, v2: Self) -> Self {
        Self (
            self.1 * v2.2 - self.2 * v2.1,
            self.2 * v2.0 - self.0 * v2.2,
            self.0 * v2.1 - self.1 * v2.0,
            0.0
        )
    }

    #[inline]
    pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self (x, y, z, w)
    }

    #[inline]
    pub fn x (self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y (self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z (self) -> f32 {
        self.2
    }

    #[inline]
    pub fn w (self) -> f32 {
        self.3
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn add(self, _rhs: Self) -> Self {
        Self (
            self.0 + _rhs.0,
            self.1 + _rhs.1,
            self.2 + _rhs.2,
            self.3 + _rhs.3
        )
    }
}

impl ops::Add<f32> for Vector {
    type Output = Self;

    #[inline]
    fn add(self, _rhs: f32) -> Self {
        Self (
            self.0 + _rhs,
            self.1 + _rhs,
            self.2 + _rhs,
            self.3 + _rhs
        )
    }
}

impl ops::AddAssign<Vector> for Vector {
    #[inline]
    fn add_assign(&mut self, _rhs: Vector) {
        *self = Self (
            self.0 + _rhs.0,
            self.1 + _rhs.1,
            self.2 + _rhs.2,
            self.3 + _rhs.3
        );
    }
}


impl ops::Sub<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn sub(self, _rhs: Self) -> Self {
        Self (
            self.0 - _rhs.0,
            self.1 - _rhs.1,
            self.2 - _rhs.2,
            self.3 - _rhs.3
        )
    }
}

impl ops::Sub<f32> for Vector {
    type Output = Self;

    #[inline]
    fn sub(self, _rhs: f32) -> Self {
        Self (
            self.0 - _rhs,
            self.1 - _rhs,
            self.2 - _rhs,
            self.3 - _rhs
        )
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, _rhs: Self) -> Self {
        Self (
            self.0 * _rhs.0,
            self.1 * _rhs.1,
            self.2 * _rhs.2,
            self.3 * _rhs.3
        )
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, _rhs: f32) -> Self {
        Self (
            self.0 * _rhs,
            self.1 * _rhs,
            self.2 * _rhs,
            self.3 * _rhs
        )
    }
}

impl ops::Mul<Matrix> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Matrix) -> Vector {
        let x = rhs.row_1.x() * self.x() + rhs.row_2.x() * self.y() + rhs.row_3.x() * self.z() + rhs.row_4.x() * 1.0;
        let y = rhs.row_1.y() * self.x() + rhs.row_2.y() * self.y() + rhs.row_3.y() * self.z() + rhs.row_4.y() * 1.0;
        let z = rhs.row_1.z() * self.x() + rhs.row_2.z() * self.y() + rhs.row_3.z() * self.z() + rhs.row_4.z() * 1.0;

        Vector::vec3(x, y, z)
    }
}

impl ops::Div<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn div(self, _rhs: Self) -> Self {
        Self (
            self.0 / _rhs.0,
            self.1 / _rhs.1,
            self.2 / _rhs.2,
            self.3 / _rhs.3
        )
    }
}

impl ops::Div<f32> for Vector {
    type Output = Self;

    #[inline]
    fn div(self, _rhs: f32) -> Self {
        Self (
            self.0 / _rhs,
            self.1 / _rhs,
            self.2 / _rhs,
            self.3 / _rhs
        )
    }
}

impl From<(f32, f32, f32, f32)> for Vector {
    fn from(tuple: (f32, f32, f32, f32)) -> Self {
        Self (tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl From<Vector> for (f32, f32, f32, f32) {
    fn from(v: Vector) -> Self {
        (v.0, v.1, v.2, v.3)
    }
}