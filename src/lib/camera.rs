//! This module defines the Camera type including its associated functions, methods and constants.

#![warn(missing_docs)]
#![allow(dead_code, missing_debug_implementations, clippy::too_many_arguments)]

use crate::image::ASPECT_RATIO_DEFAULT;
use crate::ray::Ray;
use crate::utilities::{degrees_to_radians, random_float_range};
use crate::vector::{Point3, Vec3};

const VIEWPORT_HEIGHT_DEFAULT: f64 = 2.0;
const VIEWPORT_WIDTH_DEFAULT: f64 = ASPECT_RATIO_DEFAULT * VIEWPORT_HEIGHT_DEFAULT;
const FOCAL_LENGTH_DEFAULT: f64 = 1.0;

/// The Camera type used to visualize a Scene.
pub struct Camera {
    /// Point in 3D space where the Camera will be positioned in the Scene.
    pub look_from: Point3,
    /// Point in 3D space where the Camera is pointing at.
    pub look_at: Point3,
    /// Vector in 3D space used to specify what is the 'up' direction for the Camera.
    pub view_up: Vec3,
    /// Vertical Field of View in degrees.
    pub vfov: f64,
    /// Aspect ratio of the Image.
    pub aspect_ratio: f64,
    /// Aperture of the Camera.
    pub aperture: f64,
    /// Focus ditance.
    pub focus_dist: f64,

    /// Shutter opening time.
    pub time0: f64,
    /// Shutter closing time.
    pub time1: f64,

    /// Viewport height of the Camera.
    viewport_height: f64,
    /// Viewport width of the Camera.
    viewport_width: f64,
    /// Focal length of the Camera.
    focal_length: f64,

    /// Location in 3D space where Rays are originating, which is the Viewport of the Camera.
    origin: Point3,
    /// Horizontal axis of the Viewport of the Camera.
    horizontal: Vec3,
    /// Vertical axis of the Viewport of the Camera.
    vertical: Vec3,
    /// Lower left corner of the Viewport of the Camera.
    lower_left_corner: Vec3,

    /// Orthonormal 'u' axis (horizontal) of the Camera orientation.
    u: Vec3,
    /// Orthonormal 'v' axis (vertical) of the Camera orientation.
    v: Vec3,
    /// Orthonormal 'w' axis (depth) of the Camera orientation.
    w: Vec3,
    /// Camera len's radius.
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            look_from: Point3::default(),
            look_at: Point3::default(),
            view_up: Vec3::default(),
            vfov: 0.0,
            aspect_ratio: ASPECT_RATIO_DEFAULT,
            aperture: 0.0,
            focus_dist: 0.0,
            time0: 0.0,
            time1: 0.0,

            viewport_height: VIEWPORT_HEIGHT_DEFAULT,
            viewport_width: VIEWPORT_WIDTH_DEFAULT,
            focal_length: FOCAL_LENGTH_DEFAULT,

            origin: Point3::zeroes(),
            horizontal: Vec3::new(VIEWPORT_WIDTH_DEFAULT, 0.0, 0.0),
            vertical: Vec3::new(0.0, VIEWPORT_HEIGHT_DEFAULT, 0.0),
            lower_left_corner: Point3::zeroes()
                - Vec3::new(VIEWPORT_WIDTH_DEFAULT, 0.0, 0.0) / 2.0
                - Vec3::new(0.0, VIEWPORT_HEIGHT_DEFAULT, 0.0) / 2.0
                - Vec3::new(0.0, 0.0, FOCAL_LENGTH_DEFAULT),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            lens_radius: 0.0,
        }
    }
}

impl Camera {
    /// Function that creates and returns an owned Camera.
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let view_h = 2.0 * h;
        let view_w = aspect_ratio * view_h;

        let w_t = Vec3::unit(look_from - look_at);
        let u_t = Vec3::unit(Vec3::cross(vup, w_t));
        let v_t = Vec3::cross(w_t, u_t);

        let or = look_from;
        let ho = focus_dist * view_w * u_t;
        let ve = focus_dist * view_h * v_t;
        let llc = or - ho / 2.0 - ve / 2.0 - focus_dist * w_t;
        let lr = aperture / 2.0;

        Self {
            look_from,
            look_at,
            view_up: vup,
            vfov,
            time0,
            time1,
            aspect_ratio,
            viewport_height: view_h,
            viewport_width: view_w,
            origin: or,
            horizontal: ho,
            vertical: ve,
            lower_left_corner: llc,
            lens_radius: lr,
            u: u_t,
            v: v_t,
            w: w_t,
            ..Self::default()
        }
    }

    /// Function that creates and returns an owned Ray to be used by the Camera.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            tm: random_float_range(self.time0, self.time1),
        }
    }

    /// Function that returns the aspect ratio of a Camera.
    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    /// Function that returns the Viewport height of the Camera.
    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    /// Function that returns the Viewport width of the Camera.
    pub fn viewport_width(&self) -> f64 {
        self.viewport_width
    }

    /// Function that returns the Focal length of the Camera.
    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }
}
