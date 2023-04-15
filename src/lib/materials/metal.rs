//! This module defines the Metal Material and its implementation of the Material trait.  

#![warn(missing_docs)]

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::{Materials, Scatterable};
use crate::ray::Ray;
use crate::vector::Vec3;

/// The Metal material type with the albedo and fuzz properties.
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    /// Proportion of incident light that is reflected away from the surface.
    pub albedo: Color,
    /// Proportion of 'fuzziness' of a reflection.
    pub fuzz: f64,
}
impl Metal {
    /// Function creates and returns an owned Metal material.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}
