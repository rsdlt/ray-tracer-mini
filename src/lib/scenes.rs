//! The Scenes module allows the configuration of Scenes to be rendered.  

#![warn(missing_docs)]
#![allow(missing_debug_implementations)]

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
    fn new_image() -> Self::Image;
    /// Creates a new collection of shapes for the scene.
    fn new_world() -> Self::World;
    /// Creates a new camera positioned to capture the scene.
    fn new_camera(aspect_ratio: f64) -> Self::Camera;
    /// Generates the scene that is returned to the renderer.
    fn generate_scene() -> Self::Scene;
}
