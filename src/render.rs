use crate::bitmap::Bitmap;
use crate::types::*;
use crate::ray::Ray;
use crate::hittable::*;

pub struct Raytracer {
    viewport_scale: f64,
    focal_length: f64,

    hittables: Vec<Box<dyn Hittable>>,
}

impl Raytracer {
    pub fn new() -> Self {
        Self {
            viewport_scale: 2.0,
            focal_length: 1.0,
            hittables: vec![],
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable)
    }

    pub fn render(self, image_size: Size) -> Bitmap {
        let mut bitmap = Bitmap::new(image_size);

        let viewport_height = self.viewport_scale;
        let viewport_width = image_size.aspect_ratio() * viewport_height;

        let origin = Point3::ZERO;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, self.focal_length);

        for j in 0..bitmap.height() {
            eprint!("{}{}/{} scanlines done. ({}%)", clear_line(), j, bitmap.height()-1, 100 * j / (bitmap.height()-1));
            for i in 0..bitmap.width() {
                let u = (i as f64) / (bitmap.width() as f64 - 1.0);
                let v = (j as f64) / (bitmap.height() as f64 - 1.0);

                let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
                let color = self.project(&ray);
                bitmap.set(i, j, color)
            }
        }
        eprint!("\n");

        bitmap
    }

    fn project(&self, ray: &Ray) -> Color {
        for hittable in self.hittables.iter() {
            if let Some(hit_record) = hittable.hit(ray, 0.0, std::f64::INFINITY) {
                let n = hit_record.normal.as_unit();
                return 0.5*Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
            }
        }
        self.bg_color(ray)
    }

    fn bg_color(&self, ray: &Ray) -> Color {
        let unit_direction = (ray.direction).as_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0).lerp(t, Color::new(0.5, 0.7, 1.0))
    }

}


fn clear_line() -> &'static str {
    "\x1B[2K\r"
}
