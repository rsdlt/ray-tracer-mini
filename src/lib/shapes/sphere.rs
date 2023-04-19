//! This module defines the Sphere type and its implementation of the Hittable trait.

#![warn(missing_docs)]
#![allow(missing_debug_implementations, unused_variables)]

use crate::aabb::AaBb;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::lambertian::Lambertian;
use crate::materials::Materials;
use crate::ray::Ray;
use crate::textures::solid_color::SolidColor;
use crate::textures::Texture;
use crate::utilities::PI;
use crate::vector::{Point3, Vec3};
use std::f64::consts::FRAC_2_PI;

/// A Sphere with center, radius and material.
#[derive(Debug, Clone)]
pub struct Sphere {
    /// Location of the center of the Sphere.
    center: Point3,
    /// Radius of the Sphere.
    radius: f64,
    /// Material for the Sphere.
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

    // get the sphere's texture coordinates.
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let phi = (p.z).atan2(p.x) + PI;
        let theta = (p.y).asin();

        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = theta + FRAC_2_PI / PI;
        (u, v)
    }
}
impl Default for Sphere {
    fn default() -> Self {
        let center = Point3::default();
        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
        let sphere_material = Materials::Lambertians(Lambertian::new(Texture::SolidColor(
            SolidColor::new(Color::black()),
        )));
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
        (hit_record.u, hit_record.v) = self.get_sphere_uv(outward_normal);
        hit_record.material = self.material.clone();

        Some(hit_record)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb> {
        let out_box = AaBb {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        Some(out_box)
    }
}
