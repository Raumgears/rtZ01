mod basics;
mod traits;
mod volumes;
mod utils;
mod camera;

use std::io;
use std::rc::Rc;

use crate::basics::*;
use crate::traits::*;
use crate::volumes::*;
use crate::utils::*;
use camera::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = unit_vec(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1000;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    const GAMMA: f64 = 2.0; // Gamma de base : 2.0
    let color_filter: Color = Color::new(1.0, 1.0, 1.0); // Filter de base : Color::new(1.0, 1.0, 1.0)

    // World
    let mut world = HittableList::new();

    let mat_diffus1 = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_diffus2 = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    let mat_diffus3 = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_metal = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let mat_glass = Rc::new(Dielectric::new(1.5, 0.0));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_diffus1)));
    world.add(Box::new(Cube::new(Point3::new(0.0, 0.0, -1.5), 0.5, Vec3::new(0.0, 45.0, 0.0), mat_metal)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -3.0),0.5,mat_glass)));
    world.add(Box::new(Sphere::new(Point3::new(-2.0, 0.0, -1.0),0.5, mat_diffus3)));
    world.add(Box::new(Cube::new(Point3::new(2.0, 0.0, -1.0),0.5, Vec3::new(0.0, 45.0, 45.0), mat_diffus2)));

    // Camera
    let cam = Camera::new(
        ASPECT_RATIO,
        90.0,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand_01()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand_01()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut io::stdout(), pixel_color * color_filter, SAMPLES_PER_PIXEL, GAMMA);
        }
    }
    eprint!("\nDone.\n");
}
