use rand::Rng;

// Constants

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn rand_01() -> f64 {
    // Return a random real in [0.0, 1.0[
    rand::rng().random_range(0.0..1.0)
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    // Return a random real in [min, max[
    min + (max - min) * rand_01()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}