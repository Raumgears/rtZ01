mod basics;
mod traits;
mod volumes;
mod materials;
mod utils;
mod camera;

use rayon::prelude::*;
use std::io;
use std::sync::Arc;

use crate::basics::*;
use crate::traits::*;
use crate::volumes::*;
use crate::materials::*;
use crate::utils::*;
use camera::*;

// Verify each hit for the trajectory of the ray
fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        let emitted = rec.mat.as_ref().unwrap().emitted();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return emitted + attenuation * ray_color(&scattered, world, depth + 1);
        }

        return emitted;
    }
    // Background color can be inputed here (currently sky-like or black):

    // Sky
    /* let unit_direction = unit_vec(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0) */

    // Black
    Color::new(0.0, 0.0, 0.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // Image format
    const IMAGE_WIDTH: i32 = 1000; // Horizontal Size
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 1500; // Anti-aliasing sharpness; Base Samples : 100
    const MAX_DEPTH: i32 = 50; // Number of bounces of a ray; Base Max_Depth : 50
    const GAMMA: f64 = 2.0; // Base Gamma : 2.0
    let color_filter: Color = Color::new(1.0, 1.0, 1.0); // Base Filter : Color::new(1.0, 1.0, 1.0)

    // World
    let mut world = HittableList::new();

    let mat_diffus1 = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.2)));
    let mat_diffus2 = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_metal1 = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let mat_metal2 = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.2));
    let mat_glass = Arc::new(Dielectric::new(1.5, 0.05));
    let mat_light = Arc::new(DiffuseLight::new(Color::new(10.0, 10.0, 10.0)));

    world.add(Box::new(Plane::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat_diffus1)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 3.2, 3.0), 1.5, mat_light)));
    world.add(Box::new(Cube::new(Point3::new(4.0, 0.0, 3.0), 1.0, Vec3::new(0.0, 45.0, 0.0), mat_diffus2)));
    world.add(Box::new(Cylinder::new(Point3::new(-4.0, -1.0, 3.0), 2.0, 1.0, Vec3::new(0.0, 1.0, 0.0), mat_glass)));

    // Camera
    let cam = Camera::new(
        ASPECT_RATIO,
        90.0,
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT); // .ppm header

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = ((i as f64) + rand_01()) / (IMAGE_WIDTH - 1) as f64;
                    let v = ((j as f64) + rand_01()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();
        for pixel_color in pixel_colors {
            write_color(&mut io::stdout(), pixel_color * color_filter, SAMPLES_PER_PIXEL, GAMMA);
        }
    }
    eprint!("\nDone.\n");
}
