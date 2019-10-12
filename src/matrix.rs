#![allow(dead_code)]
use std::{ops, fmt};
use crate::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    row_1: Vector,
    row_2: Vector,
    row_3: Vector,
    row_4: Vector
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            row_1: Vector::vec4(1.0, 0.0, 0.0, 0.0),
            row_2: Vector::vec4(0.0, 1.0, 0.0, 0.0),
            row_3: Vector::vec4(0.0, 0.0, 1.0, 0.0),
            row_4: Vector::vec4(0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn from_vector(v_0: Vector, v_1: Vector, v_2: Vector, v_3: Vector) -> Self {
        Self {
            row_1: v_0,
            row_2: v_1,
            row_3: v_2,
            row_4: v_3
        }
    }

    // pub fn inverse(&self) -> Self {

    // }

    pub fn determinant(&self) -> f32 {
        let (m00, m01, m02, m03) = self.row_1.into();
        let (m10, m11, m12, m13) = self.row_2.into();
        let (m20, m21, m22, m23) = self.row_3.into();
        let (m30, m31, m32, m33) = self.row_4.into();

        let m00m11 = m00 * m11;
        let m22m33 = m22 * m33;
        let m00m21 = m00 * m21;
        let m32m13 = m32 * m13;
        let m00m31 = m00 * m31;
        let m12m23 = m12 * m23;


        m00m11 * m22m33 + m00m21 * m32m13 + m00m31 * m12 * m23 +
        m10 * m01 * m32 * m23 + m10 * m21 * m02 * m33 + m10 * m31 * m22 * m03 +
        m20 * m01 * m12 * m33 + m20 * m11 * m32 * m03 + m20 * m31 * m02 * m13 +
        m30 * m01 * m22 * m13 + m30 * m11 * m02 * m23 + m30 * m21 * m12 * m03 -
        m00m11 * m32 * m23 - m00m21 * m12 * m33 - m00m31 * m22 * m13 -
        m10 * m01 * m22m33 - m10 * m21 * m32 * m03 - m10 * m31 * m02 * m23 -
        m20 * m01 * m32m13 - m20 * m11 * m02 * m33 - m20 * m31 * m12 * m03 -
        m30 * m01 * m12 * m23 - m30 * m11 * m22 * m03 - m30 * m21 * m02 * m13
    }

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
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( \n {}, \n {}, \n {}, \n {})", self.row_1, self.row_2, self.row_3, self.row_4)
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_some_stuff() {
      assert_eq!(2 + 2, 4);
    }
}
