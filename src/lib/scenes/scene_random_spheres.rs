//! This module defines a random scene that include several Sphere of different sizes and Materials.

#![warn(missing_docs)]

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::image::Image;
use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::scenes::SceneConfig;
use crate::shapes::sphere::Sphere;
use crate::utilities::{random_float, random_float_range, PI};
use crate::vector::{Point3, Vec3};

/// Type that collects a scene full of random spheres with three different materials.
pub struct RandomSpheres {
    /// Image component of the scene.
    pub image: Image,
    /// Collection of spheres and its position in the scene.
    pub world: HittableList,
    /// Camera for the scene.
    pub camera: Camera,
}

impl SceneConfig for RandomSpheres {
    type Image = Image;
    type World = HittableList;
    type Camera = Camera;
    type Scene = RandomSpheres;

    fn new_image() -> Self::Image {
        // 1 thread: 1hr 30min; let image_width = 600usize; let samples_per_pixel = 200_usize; let max_depth = 30_usize;
        Image::new(256, 256, 50, 10)
    }

    fn new_world() -> Self::World {
        // Create the ground
        let material_ground = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let shape_ground = Box::new(Sphere::new(
            Point3::new(0.0, -1_000.0, 0.0),
            1_000.0,
            material_ground,
        ));

        // Create the world scene
        let mut world = HittableList::new(shape_ground);

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
                        let sphere_material = Box::new(Lambertian::new(albedo));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));

                        world.add(sphere);
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random(0.5, 1.0);
                        let fuzz = random_float_range(0.0, 0.5);
                        let sphere_material = Box::new(Metal::new(albedo, fuzz));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));

                        world.add(sphere);
                    } else {
                        // glass
                        let sphere_material = Box::new(Dielectric::new(1.5));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));

                        world.add(sphere);
                    }
                }
            }
        }
        let material_1 = Box::new(Dielectric::new(1.5));
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            material_1,
        )));

        let material_2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material_2,
        )));

        let material_3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            material_3,
        )));

        world
    }

    fn new_camera(aspect_ratio: f64) -> Self::Camera {
        // Camera
        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;
        let _big_r = (PI / 4.0).cos();

        Camera::new(
            look_from,
            look_at,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
        )
    }

    fn generate_scene() -> Self::Scene {
        let image = Self::new_image();
        let camera = Self::new_camera(image.aspect_ratio);
        let world = Self::new_world();

        Self {
            image,
            camera,
            world,
        }
    }
}
