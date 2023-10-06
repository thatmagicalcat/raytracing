use glam::DVec3;

use crate::hittable::Hittable;

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

    pub fn ray_color<T: Hittable>(&self, world: &Vec<T>) -> DVec3 {
        if let Some(rec) = world.hit(self, 0.0..f64::INFINITY) {
            return 0.5 * (rec.normal + DVec3::new(1.0, 1.0, 1.0));
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }
}
