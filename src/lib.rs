use std::cmp::Ordering;

use hittable::{aabb::Aabb, Hittable};
use itertools::Itertools;
use rand::Rng;
use shapes::Shapes;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod shapes;

pub struct BvhNode {
    left: Box<BvhNode>,
    right: Box<BvhNode>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_objects_raw(objects: Vec<Shapes>) -> Self {
        let mut rng = rand::thread_rng();

        let axis: i32 = rng.gen_range(0..=2);

        if objects.len() == 1 {
            BvhNode {
                left: Box::new(BvhNode::from_objects_raw(
                    sorted,
                )),
                right: Box::new(BvhNode::from_objects_raw(
                    right,
                )),
                bbox,
            }
        }

        let mut sorted = objects
            .into_iter()
            .sorted_by(|a, b| box_cmp(a, b, axis))
            .collect::<Vec<Shapes>>();
        let count = sorted.len();
        let right = sorted.split_off(count / 2);
        let bbox = Aabb::from((
            sorted[..].as_ref().bounding_box(),
            right[..].as_ref().bounding_box(),
        ));
        BvhNode {
            left: Box::new(BvhNode::from_objects_raw(
                sorted,
            )),
            right: Box::new(BvhNode::from_objects_raw(
                right,
            )),
            bbox,
        }
    }
    // fn from_objects(objects: Vec<Shapes>) -> Self {
    //     let end = objects.len();
    //     BvhNode::from_objects_sized(objects, 0, end)
    // }
    //     fn from_objects_sized(
    //         objects: Vec<Shapes>,
    //         start: usize,
    //         end: usize,
    //     ) -> Self {
    //         let mut rng = rand::thread_rng();
    //         let object_span = end - start;

    //         let axis: i32 = rng.gen_range(0..=2);
    //         let comparator = if axis == 0 {
    //             box_x_compare
    //         } else {
    //             if axis == 1 {
    //                 box_y_compare
    //             } else {
    //                 box_z_compare
    //             }
    //         };

    //         todo!()
    //         //     let mut objects = objects; // Create a modifiable array of the source scene objects

    //         //     let axis: i32 = random_int(0,2);
    //         //     auto comparator = (axis == 0) ? box_x_compare
    //         //                     : (axis == 1) ? box_y_compare
    //         //                                   : box_z_compare;

    //         //     size_t object_span = end - start;

    //         //     if (object_span == 1) {
    //         //         left = right = objects[start];
    //         //     } else if (object_span == 2) {
    //         //         if (comparator(objects[start], objects[start+1])) {
    //         //             left = objects[start];
    //         //             right = objects[start+1];
    //         //         } else {
    //         //             left = objects[start+1];
    //         //             right = objects[start];
    //         //         }
    //         //     } else {
    //         //         std::sort(objects.begin() + start, objects.begin() + end, comparator);

    //         //         auto mid = start + object_span/2;
    //         //         left = make_shared<bvh_node>(objects, start, mid);
    //         //         right = make_shared<bvh_node>(objects, mid, end);
    //         //     }

    // let             bbox = Aabb::from((left.bounding_box(), right.bounding_box()));
    //     }
}

fn box_cmp<A, B>(a: A, b: B, axis_index: i32) -> Ordering
where
    A: Hittable,
    B: Hittable,
{
    a.bounding_box()
        .axis(axis_index)
        .start
        .partial_cmp(
            &b.bounding_box().axis(axis_index).start,
        )
        .unwrap()
}

fn box_compare<A, B>(a: A, b: B, axis_index: i32) -> bool
where
    A: Hittable,
    B: Hittable,
{
    return a.bounding_box().axis(axis_index).start
        < b.bounding_box().axis(axis_index).start;
}

fn box_x_compare<A, B>(a: A, b: B) -> bool
where
    A: Hittable,
    B: Hittable,
{
    return box_compare(a, b, 0);
}

fn box_y_compare<A, B>(a: A, b: B) -> bool
where
    A: Hittable,
    B: Hittable,
{
    return box_compare(a, b, 1);
}

fn box_z_compare<A, B>(a: A, b: B) -> bool
where
    A: Hittable,
    B: Hittable,
{
    return box_compare(a, b, 2);
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &ray::Ray,
        interval: std::ops::Range<f64>,
    ) -> Option<hittable::HitRecord> {
        if !self.bbox.hit(ray, interval.clone()) {
            return None;
        }

        let hit_left = self.left.hit(ray, interval.clone());
        let top = if let Some(rec) = &hit_left {
            rec.t
        } else {
            interval.end
        };
        let hit_right =
            self.right.hit(ray, interval.start..top);

        return hit_left.or(hit_right);
    }

    fn bounding_box(&self) -> hittable::aabb::Aabb {
        todo!()
    }
}
