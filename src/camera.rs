use std::fs;

use glam::DVec3;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::{thread_rng as rng, Rng};
use rayon::prelude::*;

use crate::hittable::Hittable;
use crate::ray::{random_in_unit_disk, Ray};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,

    max_depth: u32,
    center: DVec3,

    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,

    samples_per_pixel: u32,

    focus_dist: f64,
    defocus_angle: f64,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        vfov: f64,
        look_from: DVec3,
        look_at: DVec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = {
            let height = image_width as f64 / aspect_ratio;
            if height < 1.0 {
                1
            } else {
                height as _
            }
        };

        let vup = DVec3::new(0., 1., 0.);
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let center = look_from;
        let w = (look_from - look_at).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_v + pixel_delta_u);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            samples_per_pixel,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
            max_depth: 50,
        }
    }

    pub fn render_to_file<T: Hittable + std::marker::Sync>(&self, world: &Vec<T>, path: &str) {
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_par_iter()
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .map(|(y, x)| {
                let mut color = DVec3::ZERO;
                let scale = 1.0 / self.samples_per_pixel as f64;

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += ray.ray_color(self.max_depth, world);
                }

                format!(
                    "{} {} {}",
                    ((color.x * scale).sqrt().clamp(0.0, 1.0) * 255.0) as u8,
                    ((color.y * scale).sqrt().clamp(0.0, 1.0) * 255.0) as u8,
                    ((color.z * scale).sqrt().clamp(0.0, 1.0) * 255.0) as u8
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        fs::write(
            path,
            format!(
                "P3 {} {} 255\n{pixels}",
                self.image_width, self.image_height
            ),
        )
        .unwrap();
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (y as f64 * self.pixel_delta_v) + (x as f64 * self.pixel_delta_u);
        let pixel_sample = self.pixel_sample_square() + pixel_center;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> DVec3 {
        let px = -0.5 + rng().gen_range(0.0..1.0);
        let py = -0.5 + rng().gen_range(0.0..1.0);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> DVec3 {
        let p = random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }
}
