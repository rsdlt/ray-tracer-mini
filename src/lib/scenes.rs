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
/// Defines a scene with two perlin spheres
pub mod scene_two_perlin_spheres;
/// Defines a scene with two spheres touching each other.  
pub mod scene_two_spheres;

/// Dynamic dispatch of Functions that create Scenes.
type CreateWorldFunctions = Box<dyn Fn() -> HittableList>;

/// Type defining the configuration and generation of a Scene.
pub struct Scene {
    /// Image component of the scene.
    pub image: Image,
    /// Collection of spheres and its position in the scene.
    pub world: HittableList,
    /// Camera for the scene.
    pub camera: Camera,
    /// The rendered scene
    pub rendered_scene_name: String,
    /// Collection of function that create scenes.
    scenes: Vec<CreateWorldFunctions>,
}
impl Scene {
    /// Generates the scene that is returned to the renderer.
    pub fn generate_scene(config: &Config) -> Scene {
        let image = Self::create_image(&config);
        let camera = Self::set_camera(image.aspect_ratio);
        let scenes: Vec<CreateWorldFunctions> = vec![
            Box::new(scene_two_spheres::create_world),
            Box::new(scene_random_spheres::create_world),
            Box::new(scene_two_perlin_spheres::create_world),
        ];

        let selector = config.scene.clone();
        let world_creator;
        let rendered_scene_name;

        match selector.as_str() {
            "two spheres" => {
                world_creator = &scenes[0];
                rendered_scene_name = "Two Spheres".to_string()
            }

            "random spheres" => {
                world_creator = &scenes[1];
                rendered_scene_name = "Random Spheres".to_string()
            }
            "two perlin spheres" => {
                world_creator = &scenes[2];
                rendered_scene_name = "Two Perlin Spheres".to_string()
            }
            _ => panic!("wrong scene name in config file"),
        }

        let world = Self::create_world(world_creator);
        Self {
            image,
            camera,
            world,
            rendered_scene_name,
            scenes,
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
        let look_from = Point3::new(10.0, 3.0, 10.0);
        let look_at = Point3::new(20.0, 2.0, 20.0);
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
    scene: String,
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
