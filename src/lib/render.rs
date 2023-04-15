//! This module provides the functionality to render Scenes and write an Image file as output.

#![warn(missing_docs, missing_debug_implementations)]
#![allow(unused_assignments, clippy::write_with_newline)]

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;
use crate::hittable::HittableList;
use crate::materials::dielectric::Dielectric;
use crate::materials::Materials::Dielectrics;
use crate::materials::{Materials, Scatterable};
use crate::ray::Ray;
use crate::scenes::{scene_random_spheres, SceneConfig};
use crate::utilities::{random_float, INFINITY};
use crate::vector::{Point3, Vec3};

/// Render function renders a Scene and writes the result to an Image file.
pub fn render() -> Result<File, std::io::Error> {
    let scene = scene_random_spheres::RandomSpheres::generate_scene();

    // Render
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;
    let mut line = format!("P3\n{} {} \n255\n", scene.image.width, scene.image.height);

    for j in (0..scene.image.height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..scene.image.width {
            let mut pixel_color = Color::black();

            for _s in 0..scene.image.samples_per_pixel {
                let u = (i as f64 + random_float()) / (scene.image.width as f64 - 1.0);
                let v = (j as f64 + random_float()) / (scene.image.height as f64 - 1.0);
                let ray = scene.camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &scene.world, scene.image.max_depth);
            }
            Color::write_color_ppm(&mut line, &pixel_color, scene.image.samples_per_pixel);
        }
    }
    write!(img_file, "{}", line)?;

    Ok(img_file)
}

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Color {
    // Recursion base case: if exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY) {
        // diffuse render 1: let target = hit.p + hit.normal + Point3::random_in_unit_sphere();
        // diffuse render 2: let target = hit.p + hit.normal + Point3::random_unit_vector();
        // diffuse render 3:  let target = hit.p + Point3::random_in_hemisphere(&hit.normal);
        // return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);

        let mut scattered = Ray::new(Point3::default(), Vec3::default());
        let mut attenuation = Color::black();

        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::black();
    }

    let unit_direction = ray.direction().to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
