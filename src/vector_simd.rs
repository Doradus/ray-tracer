#![allow(dead_code)]
use crate::matrix::Matrix;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{ops, fmt, mem};

#[derive(Clone, Copy, Debug)]
pub struct VectorSimd (__m128);

impl VectorSimd {

    #[inline]
    pub fn vec2(x: f32, y: f32) -> Self {
        unsafe { Self (_mm_set_ps(0.0, 0.0, y, x)) }
    }

    #[inline]
    pub fn vec3(x: f32, y: f32, z: f32) -> Self {
        unsafe { Self (_mm_set_ps(0.0, z, y, x)) }
    }

    #[inline]
    pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { Self (_mm_set_ps(w, z, y, x)) }
    }

    #[inline]
    unsafe fn vec3_dot_internal(self, other: Self) -> __m128 {
        let mul = _mm_mul_ps(self.0, other.0);
        let temp = _mm_shuffle_ps(mul, mul, 0b00_00_00_01);

        let dot = _mm_add_ss(mul, temp);

        let temp = _mm_shuffle_ps(mul, mul, 0b00_00_00_10);

        let dot = _mm_add_ss(dot, temp);

        _mm_shuffle_ps(dot, dot, 0b00_00_00_00)
    }

    #[inline]
    pub fn vec3_dot(self, other: Self) -> Self {
        unsafe { Self(self.vec3_dot_internal(other))}
    }

    #[inline]
    pub fn vec3_length(self) -> Self {
        unsafe { Self(_mm_sqrt_ps(self.vec3_dot_internal(self)))}
    }

    #[inline]
    pub fn vec3_length_f32(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_sqrt_ss(self.vec3_dot_internal(self)))}
    }

//     #[inline]
//     pub fn vec3_length_reciprocal(self) -> f32 {
//         1.0 / self.vec3_length()
//     }

//     #[inline]
//     pub fn vec3_normalize(self) -> Self {
//         self * self.vec3_length_reciprocal()
//     }

//     #[inline]
//     pub fn vec3_cross(self, v2: Self) -> Self {
//         Self (
//             self.1 * v2.2 - self.2 * v2.1,
//             self.2 * v2.0 - self.0 * v2.2,
//             self.0 * v2.1 - self.1 * v2.0,
//             0.0
//         )
//     }

    #[inline]
    pub fn x (self) -> f32 {
        unsafe { _mm_cvtss_f32(self.0) }
    }

    #[inline]
    pub fn y (self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01)) }
    }

    #[inline]
    pub fn z (self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10)) }
    }

    #[inline]
    pub fn w (self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_11)) }
    }

//     #[inline]
//     pub fn set_x(&mut self, x: f32) {
//         self.0 = x;
//     }

//     #[inline]
//     pub fn set_y(&mut self, y: f32) {
//         self.1 = y;
//     }

//     #[inline]
//     pub fn set_z(&mut self, z: f32) {
//         self.2 = z;
//     }

//     #[inline]
//     pub fn set_w(&mut self, w: f32) {
//         self.3 = w;
//     }
// }

// impl fmt::Display for Vector {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
//     }
// }

// impl ops::Add<Vector> for Vector {
//     type Output = Self;

//     #[inline]
//     fn add(self, _rhs: Self) -> Self {
//         Self (
//             self.0 + _rhs.0,
//             self.1 + _rhs.1,
//             self.2 + _rhs.2,
//             self.3 + _rhs.3
//         )
//     }
// }

// impl ops::Add<f32> for Vector {
//     type Output = Self;

//     #[inline]
//     fn add(self, _rhs: f32) -> Self {
//         Self (
//             self.0 + _rhs,
//             self.1 + _rhs,
//             self.2 + _rhs,
//             self.3 + _rhs
//         )
//     }
// }

// impl ops::AddAssign<Vector> for Vector {
//     #[inline]
//     fn add_assign(&mut self, _rhs: Vector) {
//         *self = Self (
//             self.0 + _rhs.0,
//             self.1 + _rhs.1,
//             self.2 + _rhs.2,
//             self.3 + _rhs.3
//         );
//     }
// }


// impl ops::Sub<Vector> for Vector {
//     type Output = Self;

//     #[inline]
//     fn sub(self, _rhs: Self) -> Self {
//         Self (
//             self.0 - _rhs.0,
//             self.1 - _rhs.1,
//             self.2 - _rhs.2,
//             self.3 - _rhs.3
//         )
//     }
// }

// impl ops::Sub<f32> for Vector {
//     type Output = Self;

//     #[inline]
//     fn sub(self, _rhs: f32) -> Self {
//         Self (
//             self.0 - _rhs,
//             self.1 - _rhs,
//             self.2 - _rhs,
//             self.3 - _rhs
//         )
//     }
// }

// impl ops::Mul<Vector> for Vector {
//     type Output = Self;

//     #[inline]
//     fn mul(self, _rhs: Self) -> Self {
//         Self (
//             self.0 * _rhs.0,
//             self.1 * _rhs.1,
//             self.2 * _rhs.2,
//             self.3 * _rhs.3
//         )
//     }
// }

// impl ops::Mul<f32> for Vector {
//     type Output = Self;

//     #[inline]
//     fn mul(self, _rhs: f32) -> Self {
//         Self (
//             self.0 * _rhs,
//             self.1 * _rhs,
//             self.2 * _rhs,
//             self.3 * _rhs
//         )
//     }
// }

// impl ops::Mul<Matrix> for Vector {
//     type Output = Self;

//     #[inline]
//     fn mul(self, rhs: Matrix) -> Vector {
//         let x = rhs.row_1.x() * self.x() + rhs.row_2.x() * self.y() + rhs.row_3.x() * self.z() + rhs.row_4.x() * 1.0;
//         let y = rhs.row_1.y() * self.x() + rhs.row_2.y() * self.y() + rhs.row_3.y() * self.z() + rhs.row_4.y() * 1.0;
//         let z = rhs.row_1.z() * self.x() + rhs.row_2.z() * self.y() + rhs.row_3.z() * self.z() + rhs.row_4.z() * 1.0;

//         Vector::vec3(x, y, z)
//     }
// }

// impl ops::MulAssign<Vector> for Vector {
//     #[inline]
//     fn mul_assign(&mut self, _rhs: Vector) {
//         *self = Self (
//             self.0 * _rhs.0,
//             self.1 * _rhs.1,
//             self.2 * _rhs.2,
//             self.3 * _rhs.3
//         );
//     }
// }

// impl ops::MulAssign<f32> for Vector {
//     #[inline]
//     fn mul_assign(&mut self, _rhs: f32) {
//         *self = Self (
//             self.0 * _rhs,
//             self.1 * _rhs,
//             self.2 * _rhs,
//             self.3 * _rhs
//         );
//     }
// }

// impl ops::Div<Vector> for Vector {
//     type Output = Self;

//     #[inline]
//     fn div(self, _rhs: Self) -> Self {
//         Self (
//             self.0 / _rhs.0,
//             self.1 / _rhs.1,
//             self.2 / _rhs.2,
//             self.3 / _rhs.3
//         )
//     }
// }

// impl ops::Div<f32> for Vector {
//     type Output = Self;

//     #[inline]
//     fn div(self, _rhs: f32) -> Self {
//         Self (
//             self.0 / _rhs,
//             self.1 / _rhs,
//             self.2 / _rhs,
//             self.3 / _rhs
//         )
//     }
// }

// impl ops::DivAssign<f32> for Vector {
//     #[inline]
//     fn div_assign(&mut self, _rhs: f32) {
//         *self = Self (
//             self.0 / _rhs,
//             self.1 / _rhs,
//             self.2 / _rhs,
//             self.3 / _rhs
//         );
//     }
// }

// impl ops::Neg for Vector {
//     type Output = Self;

//     #[inline]
//     fn neg(self) -> Self {
//         Self (
//             -self.0,
//             -self.1,
//             -self.2,
//             -self.3
//         )
//     }
// }

// impl From<(f32, f32, f32, f32)> for Vector {
//     fn from(tuple: (f32, f32, f32, f32)) -> Self {
//         Self (tuple.0, tuple.1, tuple.2, tuple.3)
//     }
// }

// impl From<Vector> for (f32, f32, f32, f32) {
//     fn from(v: Vector) -> Self {
//         (v.0, v.1, v.2, v.3)
//     }
}