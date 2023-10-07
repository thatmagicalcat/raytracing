use std::ops::Neg;

use crate::hittable::HitRecord;
use crate::ray::{random_unit_vector, refract, Ray, reflectance};

use rand::{thread_rng as rng, Rng};
use glam::DVec3;

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: DVec3,
}

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: DVec3 },
    Metal { albedo: DVec3, fuzz: f64 },
    Dielectric { refractive_index: f64 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vector();

                if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
                    scatter_direction = hit_record.normal;
                }

                let scattered_ray = Ray::new(hit_record.point, scatter_direction);

                Some(ScatteredRay {
                    ray: scattered_ray,
                    attenuation: *albedo,
                })
            }

            Self::Metal { albedo, fuzz } => {
                let reflected_direction = reflect(&r_in.direction, &hit_record.normal);
                let scattered_ray = Ray::new(
                    hit_record.point,
                    *fuzz * random_unit_vector() + reflected_direction,
                );

                Some(ScatteredRay {
                    ray: scattered_ray,
                    attenuation: *albedo,
                })
            }

            Self::Dielectric { refractive_index } => {
                let refraction_ratio = if hit_record.front_face {
                    1.0 / *refractive_index
                } else {
                    *refractive_index
                };

                let unit_direction = r_in.direction.normalize();

                let cos_theta = unit_direction.dot(hit_record.normal).neg().min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction;
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng().gen_range(0.0..1.0) {
                    direction = reflect(&unit_direction, &hit_record.normal);
                } else {
                    direction = refract(unit_direction, hit_record.normal, refraction_ratio);
                }

                Some(ScatteredRay {
                    ray: Ray::new(hit_record.point, direction),
                    attenuation: DVec3::ONE,
                })
            }

            _ => None,
        }
    }
}

fn reflect(v: &DVec3, u: &DVec3) -> DVec3 {
    *v - 2.0 * v.dot(*u) * *u
}
