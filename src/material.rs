use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    utils::random_double,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector},
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        let scatter_direction = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(hit_rec.p, scatter_direction);
        *attenuation = self.albedo;
        (dot(scattered.direction(), hit_rec.normal)) > 0.0
    }
}

pub struct Dielectic {
    refraction_index: f64,
}

impl Dielectic {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0_squared = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectic {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // Dielectrics (glass) don't absorb light,
        // so attenuation is always white
        *attenuation = Color::new(1.0, 1.0, 1.0);

        // Determine the refraction ratio based on
        // which side of the surface we hit
        let ri = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            reflect(unit_direction, hit_rec.normal)
        } else {
            refract(unit_direction, hit_rec.normal, ri)
        };

        *scattered = Ray::new(hit_rec.p, direction);
        true
    }
}
