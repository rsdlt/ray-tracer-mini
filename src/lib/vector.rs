//! The vector module implements the functionality for Vec3 and Point3 types,
//! their transformations, associated functions and methods.

#![warn(missing_docs, missing_debug_implementations)]

use crate::color::Color;
use crate::utilities::{random_float, random_float_range, EPSILON, NEAR_ZERO};
use derive_more::{Add, Neg, Sub};
use std::ops::{Add, Div, Index, Mul};

/// Type representing a geometric 3D vector with X, Y and Z coordinates.
#[derive(Debug, Copy, Clone, Add, Sub, Neg)]
pub struct Vec3 {
    /// Component in the X coordinate.
    pub x: f64,
    /// Component in the Y coordinate.
    pub y: f64,
    /// Component in the Z coordinate.
    pub z: f64,
}

/// Type alias representing a Point in 3D with X, Y and Z components.  
pub type Point3 = Vec3;

impl Vec3 {
    /// Function that calculates and returns an owned refracted vector.
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(Vec3::dot(-uv, n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(((1.0 - r_out_perp.length_squared()).abs()).sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    /// Function that calculates and returns an owned reflected vector.
    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    /// Function returns true if the vector is close to zero in all coordinates.
    pub fn near_zero(&self) -> bool {
        self.x.abs() < NEAR_ZERO && self.y.abs() < NEAR_ZERO && self.z.abs() < NEAR_ZERO
    }

    /// Function that returns a random 3D Vector or Point3.
    pub fn random() -> Self {
        Self {
            x: random_float(),
            y: random_float(),
            z: random_float(),
        }
    }

    /// Function that returns a random 3D Vector or Point3 clamped to min and max values.
    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: random_float_range(min, max),
            y: random_float_range(min, max),
            z: random_float_range(min, max),
        }
    }

    /// Function that generates random Points inside a unit disk to accomplish Defocus Blur.
    pub fn random_in_unit_disk() -> Point3 {
        loop {
            let p = Vec3::new(
                random_float_range(-1.0, 1.0),
                random_float_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    /// Function that pics a random Point in a unit radius sphere to accomplish Diffuse Materials.
    pub fn random_in_unit_sphere() -> Point3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    /// Function that picks random Points in the surface of the unit spheres and normalizes them, to
    /// accomplish true Lambertian reflection.
    pub fn random_unit_vector() -> Self {
        Self::unit(Self::random_in_unit_sphere())
    }

    /// Function that provides an alternative diffuse formulation by providing a uniform scatter direction
    /// for angles away from hit point.
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        // check if in same hemisphere as the normal
        if Vec3::dot(in_unit_sphere, *normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Function that calculates the length squared of a vector.
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Function that calculates and returns the length of a vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Function that calculates the Dot product of two vectors.
    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// Function that calculates the Cross product of two vectors and returns a new owned vector.
    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    /// Function returns a new unit vector from another vector.
    pub fn unit(vec: Self) -> Self {
        vec / vec.length()
    }

    /// Function transform a vector to a unit vector.
    pub fn to_unit(&self) -> Self {
        *self / self.length()
    }

    /// Function creates a new vector with X, Y and Z coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Function returns a new vector filled with 'zero' values in all coordinates.  
    pub fn zeroes() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Function returns a new vector filled with 'one' values in all coordinates.  
    pub fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("wrong index for Vec3: {}", index),
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zeroes()
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        Self {
            x: value.r,
            y: value.g,
            z: value.b,
        }
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

// impl Add<Vec3> for f64 {
//     type Output = Vec3;
//
//     fn add(self, rhs: Vec3) -> Self::Output {
//         Vec3 {
//             x: self + rhs.x,
//             y: self + rhs.y,
//             z: self + rhs.z,
//         }
//     }
// }

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

        assert_eq!( Vec3::unit(v1), Vec3 { x: -1.0 / 3.0, y: 2.0 / 3.0, z: -2.0 / 3.0 } )
    }
}
