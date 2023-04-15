//! This module defines the Sphere type and its implementation of the Hittable trait.

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::lambertian::Lambertian;
use crate::materials::Materials;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

/// A Sphere with center, radius and material.
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    /// Location of the center of the Sphere.
    center: Point3,
    /// Radius of the Sphere.
    radius: f64,
    /// Material for the Sphere represented as a Trait object.
    material: Materials,
}

impl Sphere {
    /// Function returns an owned Sphere.
    pub fn new(center: Point3, radius: f64, material: Materials) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
impl Default for Sphere {
    fn default() -> Self {
        let center = Point3::default();
        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
        let sphere_material = Materials::Lambertians(Lambertian::new(albedo));
        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
        Self {
            center,
            radius: 1.0,
            material: sphere_material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();
        let origin_center = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(origin_center, ray.direction());
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material;

        Some(hit_record)
    }
}
