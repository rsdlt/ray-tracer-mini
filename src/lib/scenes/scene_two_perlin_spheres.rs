#![allow(missing_docs)]

use crate::hittable::HittableList;
use crate::materials::{lambertian::Lambertian, Materials};
use crate::shapes::sphere::Sphere;
use crate::shapes::HittableObjects;
use crate::textures::noise::Noise;
use crate::textures::Texture;
use crate::vector::Point3;

pub fn create_world() -> HittableList {
    // Create the ground
    let noise = Texture::Noise(Noise::new(4.0));
    let material_sphere = Materials::Lambertians(Lambertian::new(noise));
    let sphere_1 = HittableObjects::Sphere(Sphere::new(
        Point3::new(20.0, -1000.0, 20.0),
        1000.0,
        material_sphere.clone(),
    ));
    let sphere_2 = HittableObjects::Sphere(Sphere::new(
        Point3::new(20.0, 2.0, 20.0),
        2.0,
        material_sphere,
    ));

    // Create the world scene
    let mut world = HittableList::default();
    world.add(sphere_1);
    world.add(sphere_2);

    world
}
