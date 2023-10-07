use glam::DVec3;

use raytracing::camera::Camera;
use raytracing::material::Material;
use raytracing::sphere::Sphere;

use rand::{thread_rng, Rng};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1200;
const SAMPLES_PER_PIXEL: u32 = 500;
const VFOV: f64 = 20.0;

fn main() {
    let mut world = vec![];

    let ground_material = Material::Lambertian {
        albedo: DVec3::splat(0.5),
    };
    world.push(Sphere {
        center: DVec3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    });

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = DVec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - DVec3::new(4., 0.2, 0.)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = DVec3::new(
                        rng.gen_range(0f64..1.),
                        rng.gen_range(0f64..1.),
                        rng.gen_range(0f64..1.),
                    ) * DVec3::new(
                        rng.gen_range(0f64..1.),
                        rng.gen_range(0f64..1.),
                        rng.gen_range(0f64..1.),
                    );
                    Material::Lambertian {
                        albedo: albedo.into(),
                    }
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = DVec3::new(
                        rng.gen_range(0.5..1.),
                        rng.gen_range(0.5..1.),
                        rng.gen_range(0.5..1.),
                    );
                    let fuzz = rng.gen_range(0f64..0.5);

                    Material::Metal { albedo, fuzz }
                } else {
                    // glass
                    Material::Dielectric {
                        refractive_index: 1.5,
                    }
                };

                world.push(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }

    world.push(Sphere {
        center: DVec3::new(0., 1., 0.),
        radius: 1.0,
        material: Material::Dielectric {
            refractive_index: 1.5,
        },
    });

    world.push(Sphere {
        center: DVec3::new(-4., 1., 0.),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1).into(),
        },
    });

    world.push(Sphere {
        center: DVec3::new(4., 1., 0.),
        radius: 1.0,
        material: Material::Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    let camera = Camera::new(
        IMAGE_WIDTH,
        ASPECT_RATIO,
        SAMPLES_PER_PIXEL,
        VFOV,
        DVec3::new(13., 2., 3.),
        DVec3::new(0., 0., 0.),
        0.6,
        10.0,
    );

    camera.render_to_file(&world, "output.ppm");
}
