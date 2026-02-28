use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, reflect},
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_rec.normal + random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        *scattered = Ray::new(hit_rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction(), hit_rec.normal);
        *scattered = Ray::new(hit_rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
