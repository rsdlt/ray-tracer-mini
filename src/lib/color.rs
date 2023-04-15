//! This module defines a Color type in the Red-Green-Blue form, including its associated functions,
//! methods and transformations.  

#![warn(missing_docs, missing_debug_implementations)]

use crate::utilities::EPSILON;
use crate::vector::Vec3;
use derive_more::{Add, Neg, Sub};
use rand::prelude::*;
use std::ops::{Div, Mul};

/// The Color type in RGB form.
#[derive(Debug, Copy, Clone, Add, Sub, Neg)]
pub struct Color {
    /// Red component.
    pub r: f64,
    /// Green component.
    pub g: f64,
    /// Blue component.
    pub b: f64,
}

// implement the iterator Sum for Color so to iter().sum::<Color>()
impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Color {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        for c in iter {
            r += c.r;
            g += c.g;
            b += c.b;
        }
        Color { r, g, b }
    }
}

impl Color {
    /// Function that writes an RGB color in the form of (0,90,255) to a String in the format required
    /// by the PPM image format.
    pub fn write_color_ppm(line: &mut String, color: &Color, samples_per_pixel: usize) {
        // Divide color by # of samples and gamma-correct for gamma 2.0
        let scale = 1.0 / (samples_per_pixel as f64);
        let (r, g, b) = (
            ((color.r * scale).sqrt()).clamp(0.0, 0.999),
            ((color.g * scale).sqrt()).clamp(0.0, 0.999),
            ((color.b * scale).sqrt()).clamp(0.0, 0.999),
        );
        line.push_str(
            format!(
                "{} {} {}\n",
                (256.0 * r) as usize,
                (256.0 * g) as usize,
                (256.0 * b) as usize,
            )
            .as_str(),
        );
    }

    /// Function returns a random RGB color clamped to the min and max boundaries.
    pub fn random(min: f64, max: f64) -> Self {
        Self {
            r: thread_rng().gen_range(min.clamp(0.0, 0.999)..max.clamp(0.0, 0.999)),
            g: thread_rng().gen_range(min.clamp(0.0, 0.999)..max.clamp(0.0, 0.999)),
            b: thread_rng().gen_range(min.clamp(0.0, 0.999)..max.clamp(0.0, 0.999)),
        }
    }

    /// Function returns a new RGB color.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Function returns the white color.
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    /// Function returns the black color.
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Function returns the red color.
    pub fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }
    /// Function returns the green color.
    pub fn green() -> Self {
        Self {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    /// Function returns the blue color.
    pub fn blue() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self {
            r: value.x,
            g: value.y,
            b: value.z,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        if (self.r - other.r).abs() > EPSILON
            || (self.g - other.g).abs() > EPSILON
            || (self.g - other.g).abs() > EPSILON
        {
            return false;
        }
        true
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Div<Color> for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}
impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
impl Div<Color> for f64 {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    #[rustfmt::skip]
    fn col_derive_add() {
        let v1 = Color { r: 1.0, g: 2.0, b: 3.0, };
        let v2 = Color { r: 1.0, g: 2.0, b: 3.0, };

        assert_eq!(v1 + v2, Color { r: 2.0, g: 4.0, b: 6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn col_derive_sub() {
        let v1 = Color { r: 1.0, g: 2.0, b: 3.0, };
        let v2 = Color { r: 1.0, g: 2.0, b: 3.0, };

        assert_eq!(v1 - v2, Color { r: 0.0, g: 0.0, b: 0.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn col_mul() {
        let v1 = Color { r: 1.0, g: 2.0, b: 3.0, };
        let v2 = Color { r: 1.0, g: 2.0, b: 3.0, };

        assert_eq!(v1 * v2, Color { r: 1.0, g: 4.0, b: 9.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn col_mul_const() {
        let v1 = Color { r: 1.0, g: 2.0, b: 3.0, };
        let val = 2.0;

        assert_eq!(v1 * val, Color { r: 2.0, g: 4.0, b: 6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn col_div() {
        let v1 = Color { r: 4.0, g: 6.0, b: 9.0, };
        let v2 = Color { r: 1.0, g: 3.0, b: 3.0, };

        assert_eq!(v1 / v2, Color { r: 4.0, g: 2.0, b: 3.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn col_div_const() {
        let v1 = Color { r: 2.0, g: 4.0, b: 6.0, };
        let val = 2.0;

        assert_eq!(v1 / val, Color { r: 1.0, g: 2.0, b: 3.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn col_derive_neg() {
        let v1 = Color { r: 2.0, g: 4.0, b: 6.0, };

        assert_eq!(-v1, Color { r: -2.0, g: -4.0, b: -6.0 } )
    }
}
