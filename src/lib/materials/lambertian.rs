//! This module defines the Lambertian Material and its implementation of the Material trait.

#![warn(missing_docs)]
#![allow(unused_variables)]

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::Scatterable;
use crate::ray::Ray;

use crate::vector::Vec3;

/// The Lambertian type with the albedo property.
#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    /// Proportion of incident light that is reflected away from the surface.
    pub albedo: Color,
}
impl Lambertian {
    /// Function creates and returns an owned Lambertian material.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
