use std::sync::Arc;

use crate::{
    camera::Camera,
    color::Color,
    hittablelist::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};

mod camera;
mod color;
mod hittable;
mod hittablelist;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);

    eprintln!("\nDone");
}
