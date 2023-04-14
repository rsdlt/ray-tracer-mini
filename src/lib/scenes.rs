//! The Scenes module allows the configuration of Scenes to be rendered.  

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

/// Defines a random scene of Spheres of different sizes and material.
pub mod scene_random_spheres;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::image::Image;

// /// Scene type
// pub struct Scene {
//     /// Camera for the scene.
//     camera: Camera,
//     /// Collection containing all the shapes to be rendered in the scene.
//     world: HittableList,
//     /// Image to be rendered.
//     image: Image,
// }

/// Trait defining the configuration and generation of a Scene.
pub trait SceneConfig {
    type Scene;

    fn new_image() -> Image;
    fn new_world() -> HittableList;
    fn new_camera(aspect_ratio: f64) -> Camera;
    fn generate_scene() -> Self::Scene;
}
