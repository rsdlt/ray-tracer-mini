//! The utilities module provides helper constants and functions for computing
//! and configuring values in 3D.

#![warn(missing_docs, missing_debug_implementations)]

use rand::prelude::*;

/// Pi constant in f64 type.
pub const PI: f64 = std::f64::consts::PI;
/// Infinity constant in f64 type.
pub const INFINITY: f64 = f64::INFINITY;
/// Epsilon constant utilized for comparison of f64 values.
pub const EPSILON: f64 = 0.001;
/// Near_zero is utilized in limiting rendering computations when values approach zero.
pub const NEAR_ZERO: f64 = 1e-8;

/// Function to convert from degrees to radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Function that return f64 random values in the [0, 1) range.
pub fn random_float() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..=1.0)
}

/// Function that returns real random values in the [min, max) range.
pub fn random_float_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}

/// Function that returns usize random values in the [min, max) range.
pub fn random_usize_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}
