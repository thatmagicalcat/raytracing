use std::ops::Range;

use crate::ray::Ray;
use crate::material::Material;

use glam::DVec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material
}

impl HitRecord {
    pub fn with_face_normal(point: DVec3, t: f64, outward_normal: &DVec3, material: Material, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(*outward_normal) < 0.0;

        Self {
            material,
            point,
            t,
            front_face,
            normal: if front_face {
                *outward_normal
            } else {
                -*outward_normal
            },
        }
    }
}

impl<T> Hittable for Vec<T>
where
    T: Hittable + Sync,
{
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let (_, hit_record) = self.iter().fold((interval.end, None), |acc, item| {
            if let Some(temp_rec) = item.hit(ray, interval.start..acc.0) {
                (temp_rec.t, Some(temp_rec))
            } else {
                acc
            }
        });

        hit_record
    }
}
