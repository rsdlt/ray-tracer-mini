//! This module provides the functionality to render Scenes and write an Image file as output.

#![warn(missing_docs, missing_debug_implementations)]
#![allow(unused_assignments, clippy::write_with_newline)]

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::sync::Arc;
use thousands::Separable;

use crate::color::Color;
use crate::hittable::{Hittable, HittableList};
use crate::materials::Scatterable;
use crate::ray::Ray;
use crate::scenes::{scene_random_spheres, scene_two_spheres, Config, Scene};
use crate::utilities::{random_float, INFINITY};
use crate::vector::{Point3, Vec3};

#[derive(Debug, Clone)]
struct ScanString(String);

impl Add<ScanString> for ScanString {
    type Output = ScanString;

    fn add(self, rhs: ScanString) -> Self::Output {
        ScanString([self.0, rhs.0].join("").to_string())
    }
}

impl std::iter::Sum for ScanString {
    fn sum<I: Iterator<Item = ScanString>>(iter: I) -> ScanString {
        let mut my_string: ScanString = ScanString("".to_string());
        for ms in iter {
            my_string = my_string + ms;
        }
        my_string
    }
}
/// Render function renders a Scene and writes the result to an Image file.
pub fn render() -> Result<File, std::io::Error> {
    // Load Config
    let config = Config::load_config()?;

    // Generate scene
    let scene = Scene::generate_scene(&config);

    // Render
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;

    let mut render = ScanString(format!(
        "P3\n{} {} \n255\n",
        scene.image.width, scene.image.height
    ));

    let est_calculations = scene.image.width
        * scene.image.height
        * scene.image.max_depth
        * scene.image.samples_per_pixel;

    println!(
        "\nImage information:\n - W x H: {} x {} px\n - Recursion depth:{}\n - Samples per pixel: {}\n \
          - Number of shapes: {}\n - Estimated calculations: {}\n - Scene: {}\n\nRendering now:",
        scene.image.width,
        scene.image.height,
        scene.image.max_depth,
        scene.image.samples_per_pixel,
        scene.world.total_shapes(),
        &est_calculations.separate_with_commas(),
        scene.rendered_scene_name

    );
    let pb = ProgressBar::new(scene.image.height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
    let pb_counter = Arc::new(&pb);
    let pb_counter_iter = Arc::clone(&pb_counter);

    // --- Parallel Iteration to calculate Lines per Render Image (Returns Render)
    let pixels_height_vec = vec![ScanString("".to_string()); scene.image.height];
    render = render
        + pixels_height_vec
            .par_iter()
            .enumerate()
            .rev()
            .map(|(jdx, line)| {
                // --- Parallel Iteration to calculate Pixels per Line (returns a Scan Line)
                let pixels_width_vec = vec![ScanString("".to_string()); scene.image.width];
                let my_scan_line = pixels_width_vec
                    .par_iter()
                    .enumerate()
                    .map(|(idx, pixel)| {
                        // --- Parallel Iteration to calculate Samples per Pixel (returns a Pixel)
                        let mut pixel_color = Color::black();
                        let pixel_color_vec = vec![pixel_color; scene.image.samples_per_pixel];
                        pixel_color = pixel_color_vec
                            .par_iter()
                            .map(|color| {
                                let u = (idx as f64 + random_float())
                                    / (scene.image.width as f64 - 1.0);
                                let v = (jdx as f64 + random_float())
                                    / (scene.image.height as f64 - 1.0);
                                let ray = scene.camera.get_ray(u, v);
                                *color + ray_color(&ray, &scene.world, scene.image.max_depth)
                            })
                            .sum::<Color>();
                        // -------------
                        pixel.clone() + write_color_ppm(&pixel_color, scene.image.samples_per_pixel)
                    })
                    .sum::<ScanString>();
                //--------------
                pb_counter_iter.inc(1);
                line.clone() + my_scan_line
            })
            .sum::<ScanString>();

    pb_counter.finish();

    write!(img_file, "{}", render.0)?;

    Ok(img_file)
}

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Color {
    // Recursion base case: if exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY) {
        let mut scattered = Ray::new(Point3::default(), Vec3::default(), 0.0);
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

fn write_color_ppm(color: &Color, samples_per_pixel: usize) -> ScanString {
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
    ScanString(line)
}
