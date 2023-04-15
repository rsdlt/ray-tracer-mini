//! This module defines the HitRecord and HitTable types used to capture ray intersections with
//! shapes and collect Scenes, respectively.

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

use crate::color::Color;
use crate::materials::lambertian::Lambertian;
use crate::materials::Materials;
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::shapes::Shapes;
use crate::vector::{Point3, Vec3};

/// The HitRecord type is used to record a ray intersection with a Shape that contains an specific Material.
pub struct HitRecord {
    /// A point where a Ray hits a shape.
    pub p: Point3,
    /// Normal vector where a Ray hits a shape.
    pub normal: Vec3,
    /// Material that is applied to a shape where the Ray hits.
    pub material: Materials,
    /// "t" parameter of a Ray where it hit a shape.
    pub t: f64,
    /// Used to determine if the Ray was inside (false) or outside (true) a shape when it hit.
    pub front_face: bool,
}

impl HitRecord {
    /// Function that sets the front_face normal when a Ray hits a surface.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }

    /// Function creates and returns an owned HitRecord.
    pub fn new(p: Point3, normal: Vec3, material: Materials, t: f64, front_face: bool) -> Self {
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
        let mat = Lambertian::new(Color::black());
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            material: Materials::Lambertians(mat),
            t: 0.0,
            front_face: false,
        }
    }
}

/// Trait that defines if a type is hittable and hence can be part of a Scene and have Rays hit it.
pub trait Hittable {
    /// The hit function of a shape that implements the Hittable trait.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Structure that allows the collection of Shapes to create a Scene.
/// An instance is commonly defined as "World".
pub struct HittableList {
    /// Collection of shapes that define a Scene to be rendered.
    pub shapes: Vec<Shapes>,
}

impl HittableList {
    /// Clears the collection of any shapes.
    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    /// Adds a new shape to the collection.
    pub fn add(&mut self, shape: Shapes) {
        self.shapes.push(shape);
    }

    /// Creates and returns an empty collection of shapes.
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }

    /// Function that takes in a Ray, and "counts" a hit if "t" is between t_man and t_max boundaries.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_something: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for shape in self.shapes.iter() {
            if let Some(hit) = shape.hit(&ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_something = Some(hit)
            }
        }
        hit_something
    }
}
