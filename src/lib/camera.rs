#![allow(dead_code)]

use crate::ray::Ray;
use crate::utilities::{degrees_to_radians, ASPECT_RATIO};
use crate::vector::{Point3, Vec3};

const VIEWPORT_H: f64 = 2.0;
const VIEWPORT_W: f64 = ASPECT_RATIO * VIEWPORT_H;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub vfov: f64, // vertical field-of-view in degrees
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,

    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            look_from: Point3::default(),
            look_at: Point3::default(),
            vup: Vec3::default(),
            vfov: 0.0,
            aspect_ratio: ASPECT_RATIO,
            aperture: 0.0,
            focus_dist: 0.0,

            viewport_height: VIEWPORT_H,
            viewport_width: VIEWPORT_W,
            focal_length: FOCAL_LENGTH,

            origin: Point3::zeroes(),
            horizontal: Vec3::new(VIEWPORT_W, 0.0, 0.0),
            vertical: Vec3::new(0.0, VIEWPORT_H, 0.0),
            lower_left_corner: Point3::zeroes()
                - Vec3::new(VIEWPORT_W, 0.0, 0.0) / 2.0
                - Vec3::new(0.0, VIEWPORT_H, 0.0) / 2.0
                - Vec3::new(0.0, 0.0, FOCAL_LENGTH),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            lens_radius: 0.0,
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
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
            vup,
            vfov,
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

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    pub fn viewport_width(&self) -> f64 {
        self.viewport_width
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }
}
