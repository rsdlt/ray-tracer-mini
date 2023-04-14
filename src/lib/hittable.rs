//! This module defines the HitRecord and HitTable types used to capture ray intersections with
//! shapes and collect Scenes, respectively.

#![warn(missing_docs)]

use crate::color::Color;
use crate::materials::lambertian::Lambertian;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

/// The HitRecord struct is used to capture ray intersections with Shapes.
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }

    /// Function creates and returns an owned HitRecord.
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Box<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            material: Box::new(Lambertian::new(Color::black())),
            t: 0.0,
            front_face: false,
        }
    }
}

/// Trait that defines the hit function used by Shapes that implement this trait, to capture ray
/// intersections with them.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Structure that allows the collection of Shapes to create a Scene.
/// An instance is commonly defined as "world".
pub struct HittableList {
    /// Collection of objects that define a Scene to be rendered.
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_something: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(&ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_something = Some(hit)
            }
        }
        hit_something
    }
}
