use rand::prelude::*;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;
pub const EPSILON: f64 = 0.001;
pub const NEAR_ZERO: f64 = 1e-8;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Return real random in [0, 1)
pub fn random_float() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..=1.0)
}

// Return real random in [min, max)
pub fn random_float_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}
