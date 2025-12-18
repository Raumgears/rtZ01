# Ray Tracer in Rust

A **CPU ray tracer** written in **Rust**, based on ***Peter Shirley***'s books such as *Ray Tracing in One Weekend*, but adapted for the ***Zone01*** curriculum.

This project emphasizes **mathematical understanding** of ray tracing (normals, intersections, reflection, refraction), rather than performance or GPU optimization.

---

## Overview

- Recursive ray casting rendering
- Analytical geometric primitives
- Realistic materials (diffuse, metal, glass, ... )
- Modular architecture based on Rust traits
- Multithreading using the `rayon` crate

The project is intentionally kept simple to allow for a foundational mathematical understanding.

---

## Features

### Rendering
- Recursive ray tracing
- Anti-aliasing via multi-sampling
- Gamma correction
- Color filtering
- Configurable recursion depth

### Camera
- Perspective camera
- Explicit parameters:
  - position (`from`)
  - look-at point (`at`)
  - vertical vector (`vup`)
- Free orientation in space

---

## Materials

All materials implement the `Material` trait.

- **Lambertian**
  Probabilistic diffuse scattering (cosine-weighted)

- **Metal**
  Reflection with configurable *fuzz* parameter

- **Dielectric (glass)**
  - Refraction
  - Probabilistic reflection
  - Schlick's approximation (Fresnel)

- **Light**
  Diffuse light emission from the shape

---

## Geometry

All primitives implement the `Hittable` trait.

### Sphere
- Set of points at a distance `r` from a center point
- Correct normals with front/back face handling

### Plane
- Infinite plane defined by:
`dot(n, X) + d = 0`
- Analytical ray/plane intersection

### Disk / Square
- Defined by:
  - a plane
  - a center point
  - a radius / a size
  - an angle (for the square)

> A disk or a square is **not** an independent surface: it is a truncated plane.

### Cylinder
- Closed cylinder composed of:
  - an **analytical tube**
  - a **base disk**
  - a **top disk**
- Each part is an independent `Hittable`
- Internal composition via a primitive list

### Cube
- Cube recreated from:
  - a center point
  - a size
  - an orientation vector
- Each face has its own normal
> A cube is an independent surface, it is **not** an assembly of squares

---

## Architecture

### Main Traits
```rust
trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
```
```rust
trait Material {
    fn scatter(&self, ...) -> bool;
    fn emitted(&self) -> Color;
}
```
### HitRecord

- Stroees :
  - hit point
  - normal
  - material
  - parameter `t`
- Correct front face handling
- Normal always oriented towards the camera

---

## How to Use/Modify
You can freely modify the available geometric shapes, the scene, and the camera angle in main.rs.

Examples should already be in place to help you.

Then, in a terminal, run `cargo run > ../image.ppm`. You may use release mode for increased speed.
### Parameters
```rust
 // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // Image format
    const IMAGE_WIDTH: i32 = 1000; // Horizontal Size
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 200; // Anti-aliasing sharpness; Base Samples : 100
    const MAX_DEPTH: i32 = 50; // Number of bounces of a ray; Base Max_Depth : 50
    const GAMMA: f64 = 2.0; // Base Gamma : 2.0
    let color_filter: Color = Color::new(1.0, 1.0, 1.0); // Base Filter : Color::new(1.0, 1.0, 1.0)
```
### Scene
```rust
// World
let mut world = HittableList::new();

// Add your materials here
let green_mat = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.2)));
let mirror_mat = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));

// Add your volumes and shapes here
world.add(Box::new(Plane::new(Vec3::new(0.0, -1.0, 0.0), -0.5, green_mat.clone())));
world.add(Box::new(Cube::new(Point3::new(3.0, 0.0, 1.0), 0.5, Vec3::new(45.0, 45.0, 45.0), mirror_mat.clone())));
```
### Camera
```rust
// Camera
let cam = Camera::new(
        ASPECT_RATIO,
        90.0, // FOV, don't play too much with it in order to still see something
        Point3::new(0.0, 0.0, -1.0), // Where the camera is
        Point3::new(0.0, 0.0, 1.0), // Where the camera look
        Vec3::new(0.0, 1.0, 0.0), // Vup (do not change unless watching above/under then use Vec3::new(1.0, 0.0, 0.0))
    );

```
---

## References

- Ray Tracing in One Weekend — Peter Shirley
- Ray Tracing : The Next Week — Peter Shirley
- Ray Tracing : The Rest of Your Life — Peter Shirley

- [HUGI](hugi.scene.org) — Chris Dragan
- [SIGGRAPH Education](https://education.siggraph.org/) — Scott Owen
