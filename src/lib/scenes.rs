//! The Scenes module allows the configuration of Scenes to be rendered.  

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

use serde::Deserialize;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

/// Defines a random scene of Spheres of different sizes and material.
pub mod scene_random_spheres;

/// Trait defining the configuration and generation of a Scene.
pub trait SceneConfig {
    /// Image to be rendered.
    type Image;
    /// Collection containing all the shapes to be rendered in the scene.
    type World;
    /// Camera for the scene.
    type Camera;
    /// Scene that contains an image, world and camera and is passed to the renderer.
    type Scene;

    /// Creates a new Image for the scene.
    fn new_image() -> Result<Self::Image, std::io::Error>;
    /// Creates a new collection of shapes for the scene.
    fn new_world() -> Self::World;
    /// Creates a new camera positioned to capture the scene.
    fn new_camera(aspect_ratio: f64) -> Self::Camera;
    /// Generates the scene that is returned to the renderer.
    fn generate_scene() -> Result<Self::Scene, std::io::Error>;
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
