use std::fs;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;

use raytracing::ray::Ray;
use raytracing::sphere::Sphere;

const MAX_VALUE: u8 = 255;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = {
    let height = IMAGE_WIDTH as f64 / ASPECT_RATIO;
    if height < 1.0 {
        1
    } else {
        height as _
    }
};

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const VIEWPORT_V: DVec3 = DVec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const CAMERA_CENTER: DVec3 = DVec3::new(0.0, 0.0, 0.0);

fn main() {
    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        CAMERA_CENTER - DVec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_v + pixel_delta_u);

    // World
    let world = vec![
        Sphere {
            center: DVec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: DVec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
        .map(|(y, x)| {
            let pixel_center =
                pixel00_loc + (y as f64 * pixel_delta_v) + (x as f64 * pixel_delta_u);

            let ray_direction = pixel_center - CAMERA_CENTER;

            (Ray::new(CAMERA_CENTER, ray_direction).ray_color(&world)).as_color_string()
        })
        .join("\n");

    fs::write(
        "output.ppm",
        format!("P3 {IMAGE_WIDTH} {IMAGE_HEIGHT} {MAX_VALUE}\n{pixels}"),
    )
    .unwrap();
}

trait IntoColor {
    fn as_color_string(&self) -> String;
}

impl IntoColor for DVec3 {
    fn as_color_string(&self) -> String {
        format!(
            "{} {} {}",
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8
        )
    }
}
