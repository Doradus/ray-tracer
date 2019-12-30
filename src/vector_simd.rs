#![allow(dead_code)]
use crate::matrix::Matrix;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{ops, fmt, mem};

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z
}

#[derive(Clone, Copy, Debug)]
pub struct Vector (__m128);

impl Vector {

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
    pub fn vec3_dot_f32(self, other: Self) -> f32 {
        unsafe { _mm_cvtss_f32(self.vec3_dot_internal(other))}
    }

    #[inline]
    pub fn vec3_length(self) -> Self {
        unsafe { Self(_mm_sqrt_ps(self.vec3_dot_internal(self)))}
    }

    #[inline]
    pub fn vec3_length_f32(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_sqrt_ss(self.vec3_dot_internal(self)))}
    }

    #[inline]
    pub fn vec3_length_reciprocal(self) -> Self {
        unsafe {
            Self (_mm_div_ps(_mm_set_ps1(1.0), _mm_sqrt_ps(self.vec3_dot_internal(self))))
        }
    }

    #[inline]
    pub fn vec3_length_reciprocal_f32(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_div_ps(_mm_set_ps1(1.0), _mm_sqrt_ps(self.vec3_dot_internal(self)))) }
    }

    #[inline]
    pub fn vec3_normalize(self) -> Self {
        unsafe {
            Self (_mm_mul_ps(self.0, _mm_div_ps(_mm_set_ps1(1.0), _mm_sqrt_ps(self.vec3_dot_internal(self)))))
        }
    }

    #[inline]
    unsafe fn vec3_cross_internal(self, v2: Self) -> __m128 {
        let temp1 = _mm_shuffle_ps(self.0, self.0, 0b11_01_00_10);
        let temp2 = _mm_shuffle_ps(v2.0, v2.0, 0b11_01_00_10);

        let mul1 = _mm_mul_ps(temp1, v2.0);
        let mul2 = _mm_mul_ps(temp2, self.0);

        let sub = _mm_sub_ps(mul1, mul2);

        _mm_shuffle_ps(sub, sub, 0b11_01_00_10)
    }

    #[inline]
    pub fn vec3_cross(self, v2: Self) -> Self {
        unsafe{ Self(self.vec3_cross_internal(v2)) }
    }

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

    pub fn get_by_axis(self, axis:Axis) -> f32 {
        match axis {
            Axis::X => {
                return self.x();
            },
            Axis::Y => {
                return self.y();
            },
            Axis::Z => {
                return self.z();
            }
        }
    }

    #[inline]
    pub fn w (self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_11)) }
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        unsafe {self.0 = _mm_move_ss(self.0, _mm_set_ss(x)); }
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        unsafe { 
            let temp = _mm_move_ss(self.0, _mm_set_ss(y));
            let temp = _mm_shuffle_ps(temp, self.0, 0b11_10_00_00);
            self.0 = _mm_move_ss(temp, self.0);
         }
    }

    #[inline]
    pub fn set_z(&mut self, z: f32) {
        unsafe { 
            let temp = _mm_move_ss(self.0, _mm_set_ss(z));
            self.0 = _mm_shuffle_ps(self.0, temp, 0b11_00_01_00);
         }
    }

    #[inline]
    pub fn set_w(&mut self, w: f32) {
        unsafe { 
            let temp = _mm_move_ss(self.0, _mm_set_ss(w));
            self.0 = _mm_shuffle_ps(self.0, temp, 0b00_10_01_00);
         }
    }

    #[inline]
    pub fn splat_x(self) -> Self {
        unsafe { Self (_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }

    #[inline]
    pub fn splat_y(self) -> Self {
        unsafe { Self (_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub fn splat_z(self) -> Self {
        unsafe { Self (_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    #[inline]
    pub fn splat_w(self) -> Self {
        unsafe { Self (_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11)) }
    }

    #[inline]
    pub fn clamp(self, min: Vector, max: Vector) -> Self {
        unsafe { 
            let result = _mm_max_ps(self.0, min.0);
            let result = _mm_min_ps(result, max.0);

            Self (result)
         }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x(), self.y(), self.z(), self.w())
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn add(self, _rhs: Self) -> Self {
        unsafe {
            Self (_mm_add_ps(self.0, _rhs.0))
        }
    }
}

impl ops::Add<f32> for Vector {
    type Output = Self;

    #[inline]
    fn add(self, _rhs: f32) -> Self {
        unsafe {
            let rhs = _mm_set_ps1(_rhs);
            Self (_mm_add_ps(self.0, rhs))
        }
    }
}

impl ops::AddAssign<Vector> for Vector {
    #[inline]
    fn add_assign(&mut self, _rhs: Vector) {
        unsafe {
            *self = Self (_mm_add_ps(self.0, _rhs.0));
        }
    }
}


impl ops::Sub<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn sub(self, _rhs: Self) -> Self {
        unsafe {
            Self (_mm_sub_ps(self.0, _rhs.0))
        }
    }
}

impl ops::Sub<f32> for Vector {
    type Output = Self;

    #[inline]
    fn sub(self, _rhs: f32) -> Self {
        unsafe {
            let rhs = _mm_set_ps1(_rhs);
            Self (_mm_sub_ps(self.0, rhs))
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, _rhs: Self) -> Self {
        unsafe {
            Self (_mm_mul_ps(self.0, _rhs.0))
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, _rhs: f32) -> Self {
        unsafe {
            let rhs = _mm_set_ps1(_rhs);
            Self (_mm_mul_ps(self.0, rhs))
        }
    }
}

impl ops::MulAssign<Vector> for Vector {
    #[inline]
    fn mul_assign(&mut self, _rhs: Vector) {
        unsafe {
            *self = Self (_mm_mul_ps(self.0, _rhs.0));
        }
    }
}

impl ops::MulAssign<f32> for Vector {
    #[inline]
    fn mul_assign(&mut self, _rhs: f32) {
        unsafe {
            let rhs = _mm_set_ps1(_rhs);
            *self = Self (_mm_mul_ps(self.0, rhs));
        }
    }
}

impl ops::Div<Vector> for Vector {
    type Output = Self;

    #[inline]
    fn div(self, _rhs: Self) -> Self {
        unsafe {
            Self (_mm_div_ps(self.0, _rhs.0))
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Self;

    #[inline]
    fn div(self, _rhs: f32) -> Self {
        unsafe {
            let rhs = _mm_set_ps1(_rhs);
            Self (_mm_div_ps(self.0, rhs))
        }
    }
}

impl ops::Div<Vector> for f32 {
    type Output = Vector;

    #[inline]
    fn div(self, _rhs: Vector) -> Vector {
        unsafe {
            let lhs = _mm_set_ps1(self);
            Vector (_mm_div_ps(lhs, _rhs.0))
        }
    }
}

impl ops::DivAssign<f32> for Vector {
    #[inline]
    fn div_assign(&mut self, _rhs: f32) {
        unsafe {
            let rhs = _mm_set_ps1(_rhs);
            *self = Self (_mm_div_ps(self.0, rhs));
        }
    }
}

impl ops::Mul<Matrix> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Matrix) -> Vector {
        unsafe {
            let vec_x = _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00);
            let vec_y = _mm_shuffle_ps(self.0, self.0, 0b01_01_01_01);
            let vec_z = _mm_shuffle_ps(self.0, self.0, 0b10_10_10_10);
            let vec_w = _mm_set_ps1(1.0);

            let mul_row_1 = _mm_mul_ps(rhs.row_1.0, vec_x);
            let mul_row_2 = _mm_mul_ps(rhs.row_2.0, vec_y);
            let mul_row_3 = _mm_mul_ps(rhs.row_3.0, vec_z);
            let mul_row_4 = _mm_mul_ps(rhs.row_4.0, vec_w);

            let res_r1r2 = _mm_add_ps(mul_row_1, mul_row_2);
            let res_r1r2r3 = _mm_add_ps(res_r1r2, mul_row_3);
            let res_r1r2r3r4 = _mm_add_ps(res_r1r2r3, mul_row_4);

            Self (res_r1r2r3r4)
            // let x = rhs.row_1.x() * self.x() + rhs.row_2.x() * self.y() + rhs.row_3.x() * self.z() + rhs.row_4.x() * 1.0;
            // let y = rhs.row_1.y() * self.x() + rhs.row_2.y() * self.y() + rhs.row_3.y() * self.z() + rhs.row_4.y() * 1.0;
            // let z = rhs.row_1.z() * self.x() + rhs.row_2.z() * self.y() + rhs.row_3.z() * self.z() + rhs.row_4.z() * 1.0;
    
            // Vector::vec3(x, y, z)
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        unsafe {
            let rhs = _mm_set_ps1(-1.0);
            Self (_mm_mul_ps(self.0, rhs))
        }
    }
}

impl From<(f32, f32, f32, f32)> for Vector {
    fn from(tuple: (f32, f32, f32, f32)) -> Self {
        unsafe {
            Self (_mm_set_ps(tuple.3, tuple.2, tuple.1, tuple.0))
        }
    }
}

impl From<Vector> for (f32, f32, f32, f32) {
    fn from(v: Vector) -> Self {
        (v.x(), v.y(), v.z(), v.w())
    }
}

impl From<Vector> for [f32; 4] {
    fn from(v: Vector) -> Self {
        [v.x(), v.y(), v.z(), v.w()]
    }
}

impl From<[f32; 4]> for Vector {
    fn from(array: [f32; 4]) -> Self {
        unsafe {
            Self (_mm_set_ps(array[3], array[2], array[1], array[0]))
        }
    }
}