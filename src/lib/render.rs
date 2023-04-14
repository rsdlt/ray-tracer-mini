//! This module provides the functionality to render Scenes and write an Image file as output.

#![warn(missing_docs, missing_debug_implementations)]
#![allow(unused_assignments, clippy::write_with_newline)]

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::image::Image;
use crate::ray::Ray;
use crate::scenes::scene_random_spheres;
use crate::utilities::{random_float, INFINITY, PI};
use crate::vector::{Point3, Vec3};

/// Render function renders a Scene and writes the result to an Image file.
pub fn render() -> Result<File, std::io::Error> {
    // Camera

    // Image
    // 1 thread: 1hr 30min; let image_width = 600usize; let samples_per_pixel = 200_usize; let max_depth = 30_usize;
    let image = Image::new(256, 256, 50, 10);

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let _big_r = (PI / 4.0).cos();

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Scene, also known as 'world'
    let world = scene_random_spheres::generate();

    // Render
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;
    let mut line = format!("P3\n{} {} \n255\n", image.width, image.height);

    for j in (0..image.height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image.width {
            let mut pixel_color = Color::black();

            for _s in 0..image.samples_per_pixel {
                let u = (i as f64 + random_float()) / (image.width as f64 - 1.0);
                let v = (j as f64 + random_float()) / (image.height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, image.max_depth);
            }
            Color::write_color_ppm(&mut line, &pixel_color, image.samples_per_pixel);
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

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        // diffuse render 1: let target = hit.p + hit.normal + Point3::random_in_unit_sphere();
        // diffuse render 2: let target = hit.p + hit.normal + Point3::random_unit_vector();
        // diffuse render 3:  let target = hit.p + Point3::random_in_hemisphere(&hit.normal);
        // return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);

        let mut scattered = Ray::new(Point3::default(), Vec3::default());
        let mut attenuation = Color::black();
        if hit
            .material
            .scatter(ray, &hit, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::black();
    }

    let unit_direction = ray.direction().to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
