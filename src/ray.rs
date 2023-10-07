use std::ops::Neg;

use glam::DVec3;
use rand::{thread_rng as rng, Rng};

use crate::{hittable::Hittable, material::ScatteredRay};

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color<T: Hittable + std::marker::Sync>(&self, depth: u32, world: &Vec<T>) -> DVec3 {
        if depth <= 0 {
            return DVec3::ZERO;
        }

        if let Some(rec) = world.hit(self, (0.001)..f64::INFINITY) {
            if let Some(ScatteredRay { attenuation, ray }) = rec.material.scatter(self, &rec) {
                return attenuation * ray.ray_color(depth - 1, world);
            }

            return DVec3::ZERO;
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }
}

pub(crate) fn random_in_unit_sphere() -> DVec3 {
    loop {
        let p = DVec3::new(
            rng().gen_range(-1.0..1.),
            rng().gen_range(-1.0..1.),
            rng().gen_range(-1.0..1.),
        );

        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub(crate) fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

pub(crate) fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = uv.neg().dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: DVec3 = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * n;
    return r_out_perp + r_out_parallel;
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}

pub(crate) fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            0.,
        );

        if v.length_squared() < 1. {
            break v;
        }
    }
}