use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utils::{INFINITY, random_double},
    vec3::{Point3, Vec3, random_on_hemisphere, random_unit_vector, unit_vector},
};
use std::io::{self, Write}; // Combines io and Write

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            samples_per_pixel: 0,
            max_depth: 10,
            pixel_samples_scale: 0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:3} ", self.image_height - j);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }
        eprintln!("\rDone.                 ");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate horizontal and vertical delta vectors
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        // Use 0.001 for min value to prevent
        // floating point rounding causing
        // shadow hallucinations
        if let Some(rec) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            // // Visualize normals: map [-1, 1] to [0, 1]
            // return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));

            // // Bounce ray off object and ray
            // // retains 0.5 of intensity
            // let direction = rec.normal + random_unit_vector();
            // return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
            let mut attenuation = Color::default();
            let mut scattered = Ray::new(Point3::default(), Vec3::default());
            if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
        }
        // Blended value based on y position
        // of ray. Produces a blue to white
        // gradient.
        let unit_dir = unit_vector(ray.direction());
        let a = 0.5 * unit_dir.y() + 1.0;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating
        // from the origin and directed at
        // randomly sampled point around the
        // pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
