use super::EPSILON;

use derive_more::{Add, Neg, Sub};
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, Add, Sub, Neg)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() > EPSILON
            || (self.y - other.y).abs() > EPSILON
            || (self.y - other.y).abs() > EPSILON
        {
            return false;
        }
        true
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    #[rustfmt::skip]
    fn vec_derive_add() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        
        assert_eq!( v1 + v2, Vec3 { x: 2.0, y: 4.0, z: 6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_derive_sub() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };

        assert_eq!( v1 - v2, Vec3 { x: 0.0, y: 0.0, z: 0.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_mul() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };

        assert_eq!( v1 * v2, Vec3 { x: 1.0, y: 4.0, z: 9.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_mul_const() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let val = 2.0;

        assert_eq!( v1 * val, Vec3 { x: 2.0, y: 4.0, z: 6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_div() {
        let v1 = Vec3 { x: 4.0, y: 6.0, z: 9.0, };
        let v2 = Vec3 { x: 1.0, y: 3.0, z: 3.0, };

        assert_eq!( v1 / v2, Vec3 { x: 4.0, y: 2.0, z: 3.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_div_const() {
        let v1 = Vec3 { x: 2.0, y: 4.0, z: 6.0, };
        let val = 2.0;

        assert_eq!( v1 / val, Vec3 { x: 1.0, y: 2.0, z: 3.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_derive_neg() {
        let v1 = Vec3 { x: 2.0, y: 4.0, z: 6.0, };

        assert_eq!( -v1, Vec3 { x: -2.0, y: -4.0, z: -6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_dot() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let v2 = Vec3 { x: 4.0, y: -5.0, z: 6.0, };

        assert_eq!( Vec3::dot(v1, v2), 12.0 )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_cross() {
        let v1 = Vec3 { x: 3.0, y: -3.0, z: 1.0, };
        let v2 = Vec3 { x: 4.0, y: 9.0, z: 2.0, };

        assert_eq!( Vec3::cross(v1, v2), Vec3 { x: -15.0, y: -2.0, z: 39.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn  vec_unit() {
        let v1 = Vec3 { x: -2.0, y: 4.0, z: -4.0, };

        assert_eq!( v1.unit(), Vec3 { x: -1.0 / 3.0, y: 2.0 / 3.0, z: -2.0 / 3.0 } )
    }
}
