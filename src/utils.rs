use rand::RngExt;
use std::f64::consts::PI;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI_VAL: f64 = PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI_VAL / 180.0
}

pub fn random_double() -> f64 {
    rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
