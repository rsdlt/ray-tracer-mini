//! This module defines the Dielectric Material and its implementation of the Material trait.

#![warn(missing_docs)]
#![allow(unused_assignments)]

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::{Materials, Scatterable};
use crate::ray::Ray;
use crate::utilities::random_float;
use crate::vector::Vec3;

/// The Dielectric Material type with the Index of Refraction property.
#[derive(Copy, Clone, Debug)]
pub struct DielectricMat {
    /// Index of refraction
    pub ir: f64,
}
impl DielectricMat {
    /// Function that creates and returns an owned Dielectric Material.
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's polynomial approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
    }
}

impl Scatterable for DielectricMat {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::white();

        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_direction = r_in.direction().to_unit();
        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::default();

        if cannot_refract || DielectricMat::reflectance(cos_theta, refraction_ratio) > random_float() {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction);
        true
    }
}
