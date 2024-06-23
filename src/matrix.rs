use std::ops::{Mul,Index};
use crate::vec::Vec3;
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Default,Debug)]
pub struct Matrix33 {
    m: [f32; 9],
}

impl Index<usize> for Matrix33 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.m[i]
    }
}

impl Mul<&Vec3> for &Matrix33 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.m[0] * rhs.x() + self.m[1] * rhs.y() + self.m[2] * rhs.z(),
            self.m[3] * rhs.x() + self.m[4] * rhs.y() + self.m[5] * rhs.z(),
            self.m[6] * rhs.x() + self.m[7] * rhs.y() + self.m[8] * rhs.z(),
        )
    }
}

impl Mul<Vec3> for Matrix33 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        &self * &rhs
    }
}

impl Mul<Vec3> for &Matrix33 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self * &rhs
    }
}

impl Mul<&Vec3> for Matrix33 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        &self * rhs
    }
}

impl Mul<&Matrix33> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Matrix33) -> Vec3 {
        Vec3::new(
            self.x() * rhs[0] + self.y() * rhs[3] + self.z() * rhs[6],
            self.x() * rhs[1] + self.y() * rhs[4] + self.z() * rhs[7],
            self.x() * rhs[2] + self.y() * rhs[5] + self.z() * rhs[8],
        )
    }
}

impl Mul<&Matrix33> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Matrix33) -> Vec3 {
        &self * rhs
    }
}

impl Mul<Matrix33> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Matrix33) -> Vec3 {
        self * &rhs
    }
}

impl Mul<Matrix33> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Matrix33) -> Vec3 {
        &self * &rhs
    }
}

impl Mul<&Matrix33> for &Matrix33 {
    type Output = Matrix33;

    fn mul(self, rhs: &Matrix33) -> Matrix33 {
        Matrix33 {
            m: [self.m[0] * rhs.m[0] + self.m[1] * rhs.m[3] + self.m[2] * rhs.m[6],
                self.m[0] * rhs.m[1] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[7],
                self.m[0] * rhs.m[2] + self.m[1] * rhs.m[5] + self.m[2] * rhs.m[8],

                self.m[3] * rhs.m[0] + self.m[4] * rhs.m[3] + self.m[5] * rhs.m[6],
                self.m[3] * rhs.m[1] + self.m[4] * rhs.m[4] + self.m[5] * rhs.m[7],
                self.m[3] * rhs.m[2] + self.m[4] * rhs.m[5] + self.m[5] * rhs.m[8],

                self.m[6] * rhs.m[0] + self.m[7] * rhs.m[3] + self.m[8] * rhs.m[6],
                self.m[6] * rhs.m[1] + self.m[7] * rhs.m[4] + self.m[8] * rhs.m[7],
                self.m[6] * rhs.m[2] + self.m[7] * rhs.m[5] + self.m[8] * rhs.m[8],
            ]
        }
    }
}

impl Mul<Matrix33> for &Matrix33 {
    type Output = Matrix33;

    fn mul(self, rhs: Matrix33) -> Matrix33 {
        self * &rhs
    }
}

impl Mul<&Matrix33> for Matrix33 {
    type Output = Matrix33;

    fn mul(self, rhs: &Matrix33) -> Matrix33 {
        &self * rhs
    }
}

impl Mul<Matrix33> for Matrix33 {
    type Output = Matrix33;

    fn mul(self, rhs: Matrix33) -> Matrix33 {
        &self * &rhs
    }
}

impl Matrix33 {
    pub fn new(m: [f32; 9]) -> Matrix33 {
        Matrix33 {m}
    }

    pub fn new_default() -> Matrix33 {
        Matrix33 {
            m: [1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0]
        }
    }
}
