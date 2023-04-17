//! This module provides the functionality to render Scenes and write an Image file as output.

#![warn(missing_docs, missing_debug_implementations)]
#![allow(unused_assignments, clippy::write_with_newline)]

use crate::aabb::AaBb;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::shapes::HittableObjects;
use crate::utilities::random_usize_range;
use std::cmp::Ordering;
use std::sync::Arc;

/// Bounding Volume Hierarchy Node type
#[derive(Debug, Clone)]
pub struct BhvNode {
    /// Left node.
    left: Arc<HittableObjects>,
    /// Right node.
    right: Arc<HittableObjects>,
    /// AABB box.
    box_aabb: AaBb,
}

impl BhvNode {
    /// Creates a new BhvNode and adds it to the tree
    pub fn new(list: &mut HittableList, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let axis = random_usize_range(0, 2);

        let comparator = match axis {
            0 => BhvNode::box_x_compare,
            1 => BhvNode::box_y_compare,
            2 => BhvNode::box_z_compare,
            _ => panic!("random usize range out of bounds"),
        };

        let left;
        let right;
        let object_span = end - start;

        if object_span == 1 {
            left = list.objects[start].clone();
            right = list.objects[start].clone();
        } else if object_span == 2 {
            if BhvNode::box_compare_axis(&list.objects[start], &list.objects[start + 1], axis)
                == Ordering::Less
            {
                left = list.objects[start].clone();
                right = list.objects[start + 1].clone()
            } else {
                left = list.objects[start + 1].clone();
                right = list.objects[start].clone();
            }
        } else {
            // list.objects.sort();
            list.objects.sort_by(comparator);
            let mid = start + object_span / 2;

            left = HittableObjects::BhvNode(Self::new(list, start, mid, time0, time1));
            right = HittableObjects::BhvNode(Self::new(list, mid, end, time0, time1));
        }

        let box_left = left
            .bounding_box(time0, time1)
            .expect("No bounding box in bhv_node constructor");
        let box_right = left
            .bounding_box(time0, time1)
            .expect("No bounding box in bhv_node constructor");

        let box_aabb = AaBb::surrounding_box(box_left, box_right);

        Self {
            left: Arc::new(left),
            right: Arc::new(right),
            box_aabb,
        }
    }

    #[rustfmt::skip]
    /// Compare AaBb boxes via their axis
    pub fn box_compare_axis<'a, 'b>(a: &'a HittableObjects, b: &'b HittableObjects, axis: usize) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0).expect("No bounding box in bhd_node constructor");
        let box_b = b.bounding_box(0.0, 0.0).expect("No bounding box in bhd_node constructor");

        if box_a.min()[axis] < box_b.min()[axis] {
            Ordering::Less
        } else{ Ordering::Greater}
    }

    /// Compare AaBb boxes via their X axis
    pub fn box_x_compare<'a, 'b>(a: &'a HittableObjects, b: &'b HittableObjects) -> Ordering {
        BhvNode::box_compare_axis(&a, &b, 0)
    }
    /// Compare AaBb boxes via their Y axis
    pub fn box_y_compare<'a, 'b>(a: &'a HittableObjects, b: &'b HittableObjects) -> Ordering {
        BhvNode::box_compare_axis(&a, &b, 1)
    }
    /// Compare AaBb boxes via their Z axis
    pub fn box_z_compare<'a, 'b>(a: &'a HittableObjects, b: &'b HittableObjects) -> Ordering {
        BhvNode::box_compare_axis(&a, &b, 2)
    }

    /// Full Eq comparison of AaBb boxes.
    pub fn box_equal(a: HittableObjects, b: HittableObjects) -> bool {
        let box_a = a
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in bhd_node constructor");
        let box_b = b
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in bhd_node constructor");
        box_a.min()[0] == box_b.min()[0]
            && box_a.min()[1] == box_b.min()[1]
            && box_a.min()[2] == box_b.min()[2]
            && box_a.max()[0] == box_b.max()[0]
            && box_a.max()[1] == box_b.max()[1]
            && box_a.max()[2] == box_b.max()[2]
    }
}

impl Hittable for BhvNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.box_aabb.hit(ray, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right: Option<HitRecord>;

        if let Some(hit) = hit_left {
            hit_right = self.right.hit(ray, t_min, hit.t);
        } else {
            hit_right = self.right.hit(ray, t_min, t_max);
        }
        hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AaBb> {
        Some(self.box_aabb)
    }
}

impl Eq for HittableObjects {}

impl Ord for HittableObjects {
    fn cmp(&self, other: &Self) -> Ordering {
        let box_a = self
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in bhd_node constructor");
        let box_b = other
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in bhd_node constructor");

        if box_a.min()[0] < box_b.min()[0] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd<Self> for HittableObjects {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let box_a = self
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in bhd_node constructor");
        let box_b = other
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in bhd_node constructor");

        if box_a.min()[0] < box_b.min()[0] {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq<Self> for HittableObjects {
    fn eq(&self, other: &Self) -> bool {
        BhvNode::box_equal(self.clone(), other.clone())
    }
}
