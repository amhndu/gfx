use std::rc::Rc;

use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec3},
};

pub enum ScatterResult {
    Absorbed,
    Scattered { scattered: Ray, attenuation: Color },
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> ScatterResult;
}

pub struct ApproxLambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

pub struct Dielectric {
    refraction_index: f64,
}

pub struct AltLambertian {
    albedo: Color,
}

impl ApproxLambertian {
    pub fn new(albedo: Color) -> Rc<Self> {
        Rc::new(Self { albedo })
    }
}

impl Material for ApproxLambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> ScatterResult {
        let mut direction = hit.normal + Vec3::random_unit_vector();
        // check for degenerate scatter condition (if the normal and the random vector are exactly opposite)
        if direction.is_near_zero() {
            direction = hit.normal;
        }
        let ray = Ray::new(hit.point, direction);
        ScatterResult::Scattered {
            scattered: ray,
            attenuation: self.albedo,
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Rc<Self> {
        Rc::new(Self {
            albedo,
            fuzz: fuzz.min(1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> ScatterResult {
        let reflected = ray.direction.as_unit().reflect(hit.normal);
        let direction = reflected + self.fuzz * Vec3::random_unit_vector();
        if direction.dot(hit.normal) < 0.0 {
            return ScatterResult::Absorbed;
        }

        let ray = Ray::new(hit.point, direction);
        ScatterResult::Scattered {
            scattered: ray,
            attenuation: self.albedo,
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Rc<Self> {
        Rc::new(Self { refraction_index })
    }

    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> ScatterResult {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.as_unit();
        let cosine = (-unit_direction).dot(hit.normal).min(1.0);
        let sine = (1.0 - cosine * cosine).sqrt();

        let cannot_refract = refraction_ratio * sine > 1.0;
        let direction = if cannot_refract || Self::reflectance(cosine, refraction_ratio) > random()
        {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, refraction_ratio)
        };

        let ray = Ray::new(hit.point, direction);

        let attenuation = Color::new(1.0, 1.0, 1.0);
        ScatterResult::Scattered {
            scattered: ray,
            attenuation,
        }
    }
}

impl AltLambertian {
    pub fn new(albedo: Color) -> Rc<Self> {
        Rc::new(Self { albedo })
    }
}

impl Material for AltLambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> ScatterResult {
        let in_unit_sphere = Vec3::random_unit_vector();
        let mut direction = if in_unit_sphere.dot(hit.normal) > 0.0 {
            // In the same hemisphere as the normal
            hit.point + in_unit_sphere
        } else {
            hit.point - in_unit_sphere
        };
        // check for degenerate scatter condition (if the normal and the random vector are exactly opposite)
        if direction.is_near_zero() {
            direction = hit.normal;
        }
        let ray = Ray::new(hit.point, direction - hit.point);
        ScatterResult::Scattered {
            scattered: ray,
            attenuation: self.albedo,
        }
    }
}
