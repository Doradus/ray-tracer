#![allow(dead_code)]
use std::{ops, fmt};

use crate::vector::Vector;

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

    pub fn inverse(&self) -> Self {

    }

    pub fn determinant(&self) -> Self {

    }

    pub fn transpose(&self) -> Self {

    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, \n {}, \n {}, \n {})", self.row_1, self.row_2, self.row_3, self.row_4)
    }
}
