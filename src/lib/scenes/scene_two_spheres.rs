#![allow(missing_docs)]

use crate::color::Color;
use crate::hittable::HittableList;
use crate::materials::{lambertian::Lambertian, Materials};
use crate::shapes::sphere::Sphere;
use crate::shapes::HittableObjects;
use crate::textures::checker::Checker;
use crate::textures::Texture;
use crate::vector::Point3;

pub struct TwoSpheres;

impl TwoSpheres {
    pub fn create_world() -> HittableList {
        // Create the ground
        let checker_texture = Texture::Checker(Checker::new(
            Color::new(0.0, 0.0, 0.0),
            Color::new(0.9, 0.9, 0.9),
        ));
        let material_sphere = Materials::Lambertians(Lambertian::new(checker_texture));
        let sphere_1 = HittableObjects::Sphere(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            material_sphere.clone(),
        ));
        let sphere_2 = HittableObjects::Sphere(Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            material_sphere,
        ));

        // Create the world scene
        let mut world = HittableList::default();
        world.add(sphere_1);
        world.add(sphere_2);

        world
    }
}
