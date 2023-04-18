//! The Scenes module allows the configuration of Scenes to be rendered.  

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::image::Image;
use crate::utilities::PI;
use crate::vector::{Point3, Vec3};
use serde::Deserialize;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

/// Defines a random scene of Spheres of different sizes and material.
pub mod scene_random_spheres;
/// Defines a scene with two spheres touching each other.  
pub mod scene_two_spheres;

/// Type defining the configuration and generation of a Scene.
pub struct Scene {
    /// Image component of the scene.
    pub image: Image,
    /// Collection of spheres and its position in the scene.
    pub world: HittableList,
    /// Camera for the scene.
    pub camera: Camera,
}
impl Scene {
    /// Generates the scene that is returned to the renderer.
    pub fn generate_scene<F: FnOnce() -> HittableList>(config: &Config, world_creator: F) -> Scene {
        let image = Self::create_image(&config);
        let camera = Self::set_camera(image.aspect_ratio);
        let world = Self::create_world(world_creator);

        Self {
            image,
            camera,
            world,
        }
    }
    fn create_image(config: &Config) -> Image {
        Image::new(
            config.img_width,
            config.img_height,
            config.samples,
            config.depth,
        )
    }
    fn set_camera(aspect_ratio: f64) -> Camera {
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
            0.0,
            1.0,
        )
    }
    fn create_world<F: FnOnce() -> HittableList>(configurator: F) -> HittableList {
        let world = configurator();
        world
    }
}

/// Type used to load the config.toml configuration.
#[derive(Deserialize, Debug)]
pub struct Config {
    img_width: usize,
    img_height: usize,
    depth: usize,
    samples: usize,
}

impl Config {
    pub(crate) fn load_config() -> Result<Config, std::io::Error> {
        let path = Path::new("config.toml");

        let mut config_file = File::open(path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let config_toml = toml::from_str(buffer.as_str());
        match config_toml {
            Ok(config) => Ok(config),
            Err(err) => Err(std::io::Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}
