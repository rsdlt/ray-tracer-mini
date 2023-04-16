//! This module defines the Moving Sphere type and its implementation of the Hittable trait.

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

use crate::aabb::AaBb;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::lambertian::Lambertian;
use crate::materials::Materials;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

/// A Moving Sphere with center, radius and material.
#[derive(Debug, Clone, Copy)]
pub struct MovingSphere {
    /// Center location of the moving sphere at time0
    center0: Point3,
    /// Center location of the moving sphere at time0
    center1: Point3,
    /// Initial time movement for the sphere.
    time0: f64,
    /// End time movement for the sphere.
    time1: f64,
    /// Radius of the Sphere.
    radius: f64,
    /// Material for the moving Sphere.
    material: Materials,
}

impl Default for MovingSphere {
    fn default() -> Self {
        let center0 = Point3::default();
        let center1 = Point3::default();
        let time0 = 0.0;
        let time1 = 0.0;
        let radius = 1.0;
        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
        let sphere_material = Materials::Lambertians(Lambertian::new(albedo));
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: sphere_material,
        }
    }
}

impl MovingSphere {
    /// Creates and returns an owned moving sphere.
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Materials,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    /// Returns the center of a moving sphere.
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();
        let origin_center = ray.origin() - self.center(ray.time());
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
        let outward_normal = (hit_record.p - self.center(ray.time())) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material;

        Some(hit_record)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb> {
        let box0 = AaBb::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AaBb::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let output_box = AaBb::surrounding_box(box0, box1);
        Some(output_box)
    }
}
