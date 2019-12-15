#![allow(dead_code)]
use std::{ops, fmt};
use crate::vector_simd::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    pub row_1: Vector,
    pub row_2: Vector,
    pub row_3: Vector,
    pub row_4: Vector
}

impl Matrix {
    #[inline]
    pub fn identity() -> Self {
        Self {
            row_1: Vector::vec4(1.0, 0.0, 0.0, 0.0),
            row_2: Vector::vec4(0.0, 1.0, 0.0, 0.0),
            row_3: Vector::vec4(0.0, 0.0, 1.0, 0.0),
            row_4: Vector::vec4(0.0, 0.0, 0.0, 1.0)
        }
    }

    #[inline]
    pub fn from_vector(v_0: Vector, v_1: Vector, v_2: Vector, v_3: Vector) -> Self {
        Self {
            row_1: v_0,
            row_2: v_1,
            row_3: v_2,
            row_4: v_3
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        let (m00, m01, m02, m03) = self.row_1.into();
        let (m10, m11, m12, m13) = self.row_2.into();
        let (m20, m21, m22, m23) = self.row_3.into(); 
        let (m30, m31, m32, m33) = self.row_4.into();

        let a = m22 * m33 - m23 * m32;
        let b = m12 * m33 - m13 * m32;
        let c = m12 * m23 - m13 * m22;
        let d = m02 * m33 - m03 * m32;
        let e = m02 * m23 - m03 * m22;
        let f = m02 * m13 - m03 * m12;

        m00 * (m11 * a - m21 * b + m31 * c) -
			m10 * (m01 * a - m21 * d + m31 * e) +
			m20 * (m01 * b - m11 * d + m31 * f) -
			m30 * (m01 * c - m11 * e + m21 * f)
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let (m00, m01, m02, m03) = self.row_1.into();
        let (m10, m11, m12, m13) = self.row_2.into();
        let (m20, m21, m22, m23) = self.row_3.into(); 
        let (m30, m31, m32, m33) = self.row_4.into();

        let a = m22 * m33 - m23 * m32;
        let b = m12 * m33 - m13 * m32;
        let c = m12 * m23 - m13 * m22;
        let d = m02 * m33 - m03 * m32;
        let e = m02 * m23 - m03 * m22;
        let f = m02 * m13 - m03 * m12;
        let g = m21 * m33 - m23 * m31;
        let h = m11 * m33 - m13 * m31;
        let i = m11 * m23 - m13 * m21;
        let j = m21 * m32 - m22 * m31;
        let k = m11 * m32 - m12 * m31;
        let l = m11 * m22 - m12 * m21;
        let m = m01 * m33 - m03 * m31;
        let n = m01 * m23 - m03 * m21;
        let o = m01 * m13 - m03 * m11;
        let p = m01 * m32 - m02 * m31;
        let q = m01 * m22 - m02 * m21;
        let r = m01 * m12 - m02 * m11;

        let det1 = m11 * a - m21 * b + m31 * c;
        let det2 = m01 * a - m21 * d + m31 * e;
        let det3 = m01 * b - m11 * d + m31 * f;
        let det4 = m01 * c - m11 * e + m21 * f;

        let det = m00 * det1 -
			m10 * det2 +
			m20 * det3 -
			m30 * det4;
        
        let r_det = 1.0 / det;

        let result_00 = r_det * det1;

        let result_01 = -r_det * (
            m10 * a -
            m20 * b +
            m30 * c
        );

       let result_02 = r_det * (
            m10 * g -
            m20 * h +
            m30 * i
        );

        let result_03 = -r_det * (
            m10 * j -
            m20 * k +
            m30 * l
        );

        let result_10 = -r_det * det2;

        let result_11 = r_det * (
            m00 * a -
            m20 * d +
            m30 * e
        );

        let result_12 = -r_det * (
            m00 * g -
            m20 * m +
            m30 * n
        );

        let result_13 = r_det * (
            m00 * j -
            m20 * p +
            m30 * q
        );

        let result_20 = r_det * det3;

        let result_21 = -r_det * (
            m00 * b -
            m10 * d +
            m30 * f
        );

        let result_22 = r_det * (
            m00 * h -
            m10 * m +
            m30 * o
        );

        let result_23 = -r_det * (
            m00 * k -
            m10 * p +
            m30 * r
        );

        let result_30 = -r_det * det4;

        let result_31 = r_det * (
            m00 * c -
            m10 * e +
            m20 * f
        );

        let result_32 = -r_det * (
            m00 * i -
            m10 * n +
            m20 * o
        );

        let result_33 = r_det * (
            m00 * l -
            m10 * q +
            m20 * r
        );

        Self {
            row_1: Vector::vec4(result_00, result_10, result_20, result_30),
            row_2: Vector::vec4(result_01, result_11, result_21, result_31),
            row_3: Vector::vec4(result_02, result_12, result_22, result_32),
            row_4: Vector::vec4(result_03, result_13, result_23, result_33),
        }
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let (m00, m01, m02, m03) = self.row_1.into();
        let (m10, m11, m12, m13) = self.row_2.into();
        let (m20, m21, m22, m23) = self.row_3.into();
        let (m30, m31, m32, m33) = self.row_4.into();

        Self {
            row_1: Vector::vec4(m00, m10, m20, m30),
            row_2: Vector::vec4(m01, m11, m21, m31),
            row_3: Vector::vec4(m02, m12, m22, m32),
            row_4: Vector::vec4(m03, m13, m23, m33),
        }
    }

    #[inline]
    pub fn scaling_matrix(v:Vector) -> Self {
        Self {
            row_1: Vector::vec4(v.x(), 0.0, 0.0, 0.0),
            row_2: Vector::vec4(0.0, v.y(), 0.0, 0.0),
            row_3: Vector::vec4(0.0, 0.0, v.z(), 0.0),
            row_4: Vector::vec4(0.0, 0.0, 0.0, 1.0)
        }
    }

    #[inline]
    pub fn translation_matrix(t: Vector) -> Self {
        Self {
            row_1: Vector::vec4(1.0, 0.0, 0.0, 0.0),
            row_2: Vector::vec4(0.0, 1.0, 0.0, 0.0),
            row_3: Vector::vec4(0.0, 0.0, 1.0, 0.0),
            row_4: Vector::vec4(t.x(), t.y(), t.z(), 1.0)
        }
    }

    #[inline]
    pub fn roatation_x(rotation: f32) -> Self {
        Self {
            row_1: Vector::vec4(1.0, 0.0, 0.0, 0.0),
            row_2: Vector::vec4(0.0, rotation.cos(), rotation.sin(), 0.0),
            row_3: Vector::vec4(0.0, -rotation.sin(), rotation.cos(), 0.0),
            row_4: Vector::vec4(0.0, 0.0, 0.0, 1.0)
        }
    }

    #[inline]
    pub fn roatation_y(rotation: f32) -> Self {
        Self {
            row_1: Vector::vec4(rotation.cos(), 0.0, -rotation.sin(), 0.0),
            row_2: Vector::vec4(0.0, 1.0, 0.0, 0.0),
            row_3: Vector::vec4(rotation.sin(), 0.0, rotation.cos(), 0.0),
            row_4: Vector::vec4(0.0, 0.0, 0.0, 1.0)
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( \n {}, \n {}, \n {}, \n {})", self.row_1, self.row_2, self.row_3, self.row_4)
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let x = self.row_1.x();
        let y = self.row_1.y();
        let z = self.row_1.z();
        let w = self.row_1.w();

        let row_1 = Vector::vec4(
            x * rhs.row_1.x() + y * rhs.row_2.x() + z * rhs.row_3.x() + w * rhs.row_4.x(),
            x * rhs.row_1.y() + y * rhs.row_2.y() + z * rhs.row_3.y() + w * rhs.row_4.y(),
            x * rhs.row_1.z() + y * rhs.row_2.z() + z * rhs.row_3.z() + w * rhs.row_4.z(),
            x * rhs.row_1.w() + y * rhs.row_2.w() + z * rhs.row_3.w() + w * rhs.row_4.w()
        ); 

        let x = self.row_2.x();
        let y = self.row_2.y();
        let z = self.row_2.z();
        let w = self.row_2.w();

        let row_2 = Vector::vec4(
            x * rhs.row_1.x() + y * rhs.row_2.x() + z * rhs.row_3.x() + w * rhs.row_4.x(),
            x * rhs.row_1.y() + y * rhs.row_2.y() + z * rhs.row_3.y() + w * rhs.row_4.y(),
            x * rhs.row_1.z() + y * rhs.row_2.z() + z * rhs.row_3.z() + w * rhs.row_4.z(),
            x * rhs.row_1.w() + y * rhs.row_2.w() + z * rhs.row_3.w() + w * rhs.row_4.w()
        ); 

        let x = self.row_3.x();
        let y = self.row_3.y();
        let z = self.row_3.z();
        let w = self.row_3.w();

        let row_3 = Vector::vec4(
            x * rhs.row_1.x() + y * rhs.row_2.x() + z * rhs.row_3.x() + w * rhs.row_4.x(),
            x * rhs.row_1.y() + y * rhs.row_2.y() + z * rhs.row_3.y() + w * rhs.row_4.y(),
            x * rhs.row_1.z() + y * rhs.row_2.z() + z * rhs.row_3.z() + w * rhs.row_4.z(),
            x * rhs.row_1.w() + y * rhs.row_2.w() + z * rhs.row_3.w() + w * rhs.row_4.w()
        ); 

        let x = self.row_4.x();
        let y = self.row_4.y();
        let z = self.row_4.z();
        let w = self.row_4.w();

        let row_4 = Vector::vec4(
            x * rhs.row_1.x() + y * rhs.row_2.x() + z * rhs.row_3.x() + w * rhs.row_4.x(),
            x * rhs.row_1.y() + y * rhs.row_2.y() + z * rhs.row_3.y() + w * rhs.row_4.y(),
            x * rhs.row_1.z() + y * rhs.row_2.z() + z * rhs.row_3.z() + w * rhs.row_4.z(),
            x * rhs.row_1.w() + y * rhs.row_2.w() + z * rhs.row_3.w() + w * rhs.row_4.w()
        ); 

        Self {
            row_1,
            row_2,
            row_3,
            row_4
        }
    }
} 