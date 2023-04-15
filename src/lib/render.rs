//! This module provides the functionality to render Scenes and write an Image file as output.

#![warn(missing_docs, missing_debug_implementations)]
#![allow(unused_assignments, clippy::write_with_newline)]

use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;

use crate::color::Color;
use crate::hittable::HittableList;
use crate::materials::Scatterable;
use crate::ray::Ray;
use crate::scenes::{scene_random_spheres, SceneConfig};
use crate::utilities::{random_float, INFINITY};
use crate::vector::{Point3, Vec3};

#[derive(Debug, Clone)]
struct MyScanLine(String);

impl Add<MyScanLine> for MyScanLine {
    type Output = MyScanLine;

    fn add(self, rhs: MyScanLine) -> Self::Output {
        MyScanLine([self.0, rhs.0].join("").to_string())
    }
}

impl std::iter::Sum for MyScanLine {
    fn sum<I: Iterator<Item = MyScanLine>>(iter: I) -> MyScanLine {
        let mut my_string: MyScanLine = MyScanLine("".to_string());
        for ms in iter {
            my_string = my_string + ms;
        }
        my_string
    }
}
fn ray_string() -> MyScanLine {
    MyScanLine("new token\n".to_string())
}

/// Render function renders a Scene and writes the result to an Image file.
pub fn render() -> Result<File, std::io::Error> {
    let scene = scene_random_spheres::RandomSpheres::generate_scene();

    // Render
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;

    let mut scan_line = format!("P3\n{} {} \n255\n", scene.image.width, scene.image.height);

    for j in (0..scene.image.height).rev() {
        println!("Scan lines remaining: {}", j);

        let mut scan_line_col = vec![MyScanLine("".to_string()); scene.image.width];

        let my_scan_line = scan_line_col
            .par_iter()
            .enumerate()
            .map(|(idx, line)| {
                // --- Parallel Iteration to calculate Samples per Pixel.
                let mut pixel_color = Color::black();
                let pixel_color_col = vec![pixel_color; scene.image.samples_per_pixel];
                pixel_color = pixel_color_col
                    .par_iter()
                    .map(|color| {
                        let u = (idx as f64 + random_float()) / (scene.image.width as f64 - 1.0);
                        let v = (j as f64 + random_float()) / (scene.image.height as f64 - 1.0);
                        let ray = scene.camera.get_ray(u, v);
                        *color + ray_color(&ray, &scene.world, scene.image.max_depth)
                    })
                    .sum::<Color>();
                // -------------

                line.clone() + write_color_ppm(&pixel_color, scene.image.samples_per_pixel)
            })
            .sum::<MyScanLine>();

        scan_line.push_str(my_scan_line.0.as_str())
        //--------------
    }

    write!(img_file, "{}", scan_line)?;

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

fn write_color_ppm(color: &Color, samples_per_pixel: usize) -> MyScanLine {
    // Divide color by # of samples and gamma-correct for gamma 2.0
    let scale = 1.0 / (samples_per_pixel as f64);
    let (r, g, b) = (
        ((color.r * scale).sqrt()).clamp(0.0, 0.999),
        ((color.g * scale).sqrt()).clamp(0.0, 0.999),
        ((color.b * scale).sqrt()).clamp(0.0, 0.999),
    );
    let mut line = String::new();
    line.push_str(
        format!(
            "{} {} {}\n",
            (256.0 * r) as usize,
            (256.0 * g) as usize,
            (256.0 * b) as usize,
        )
        .as_str(),
    );
    MyScanLine(line)
}
