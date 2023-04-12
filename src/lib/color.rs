use crate::utilities::EPSILON;
use crate::vector::Vec3;
use derive_more::{Add, Neg, Sub};
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, Add, Sub, Neg)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn write_color(line: &mut String, color: &Color, samples_per_pixel: usize) {
        let scale = 1.0 / (samples_per_pixel as f64);
        line.push_str(
            format!(
                "{} {} {}\n",
                (256.0 * (color.r * scale).clamp(0.0, 0.999)) as usize,
                (256.0 * (color.g * scale).clamp(0.0, 0.999)) as usize,
                (256.0 * (color.b * scale).clamp(0.0, 0.999)) as usize,
            )
            .as_str(),
        );
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn green() -> Self {
        Self {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }
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
