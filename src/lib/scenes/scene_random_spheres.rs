#![allow(missing_docs)]

use crate::color::Color;
use crate::hittable::HittableList;
use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Materials};
use crate::shapes::moving_sphere::MovingSphere;
use crate::shapes::sphere::Sphere;
use crate::shapes::HittableObjects;
use crate::textures::checker::Checker;
use crate::textures::solid_color::SolidColor;
use crate::textures::Texture;
use crate::utilities::{random_float, random_float_range};
use crate::vector::{Point3, Vec3};

pub struct RandomSpheres;

impl RandomSpheres {
    pub fn create_world() -> HittableList {
        // Create the ground
        let checker_texture = Texture::Checker(Checker::new(
            Color::new(0.0, 0.0, 0.0),
            Color::new(0.9, 0.9, 0.9),
        ));
        let material_ground = Materials::Lambertians(Lambertian::new(checker_texture));
        let sphere_ground = HittableObjects::Sphere(Sphere::new(
            Point3::new(0.0, -1_000.0, 0.0),
            1_000.0,
            material_ground,
        ));

        // Create the world scene
        let mut world = HittableList::default();
        world.add(sphere_ground);

        // Create the scene
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_float();
                let center = Point3::new(
                    a as f64 + 0.9 * random_float(),
                    0.2,
                    b as f64 + 0.9 * random_float(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                        let sphere_material = Materials::Lambertians(Lambertian::new(
                            Texture::SolidColor(SolidColor::new(albedo)),
                        ));
                        let center2 = center + Vec3::new(0.0, random_float_range(0.0, 0.5), 0.0);
                        let moving_sphere = HittableObjects::MovingSphere(MovingSphere::new(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        ));

                        world.add(moving_sphere);
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random(0.5, 1.0);
                        let fuzz = random_float_range(0.0, 0.5);
                        let sphere_material = Materials::Metals(Metal::new(albedo, fuzz));
                        let sphere =
                            HittableObjects::Sphere(Sphere::new(center, 0.2, sphere_material));

                        world.add(sphere);
                    } else {
                        // glass
                        let sphere_material = Materials::Dielectrics(Dielectric::new(1.5));
                        let sphere =
                            HittableObjects::Sphere(Sphere::new(center, 0.2, sphere_material));

                        world.add(sphere);
                    }
                }
            }
        }
        let material_1 = Materials::Dielectrics(Dielectric::new(1.5));
        world.add(HittableObjects::Sphere(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            material_1,
        )));

        let material_2 = Materials::Lambertians(Lambertian::new(Texture::SolidColor(
            SolidColor::new(Color::new(0.4, 0.2, 0.1)),
        )));
        world.add(HittableObjects::Sphere(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material_2,
        )));

        let material_3 = Materials::Metals(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(HittableObjects::Sphere(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            material_3,
        )));

        world
    }
}
