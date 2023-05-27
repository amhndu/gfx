use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}

pub struct HitResult {
    pub record: HitRecord,
    pub material: Rc<dyn Material>,
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub t: f64,
    // normal is always against the ray
    pub normal: Vec3,
    // whether the ray hit the front face
    pub front_face: bool,
}

pub struct World {
    pub hittables: Vec<Rc<dyn Hittable>>,
}

#[derive(Clone)]
pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl World {
    pub fn new() -> World {
        World { hittables: vec![] }
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.hittables.push(hittable)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let nearest_hit = self
            .hittables
            .iter()
            .filter_map(|hittable| hittable.hit(ray, t_min, t_max))
            .min_by(|a, b| a.record.t.partial_cmp(&b.record.t).unwrap());

        nearest_hit
    }
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64, material: Rc<dyn Material>) -> Rc<Self> {
        Rc::new(Self {
            centre,
            radius,
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_sq();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        // if the first root is out of range, try the second one
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let out_normal = (hit_point - self.centre) / self.radius;
        HitResult::new(
            HitRecord::from_outward_normal(root, hit_point, ray.direction, out_normal),
            Rc::clone(&self.material),
        )
    }
}

impl HitResult {
    pub fn new(record: HitRecord, material: Rc<dyn Material>) -> Option<Self> {
        Self { record, material }.into()
    }
}

impl HitRecord {
    pub fn from_outward_normal(t: f64, point: Point3, ray: Vec3, out_normal: Vec3) -> Self {
        let front_face = ray.dot(out_normal) < 0.0;
        let normal = if front_face { out_normal } else { -out_normal };
        Self {
            point,
            t,
            front_face,
            normal,
        }
    }
}
