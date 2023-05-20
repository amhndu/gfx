use crate::bitmap::Bitmap;
use crate::types::*;
use crate::ray::Ray;

pub struct Raytracer {
    viewport_scale: f64,
    focal_length: f64,
}

impl Raytracer {
    pub fn new() -> Self {
        Self {
            viewport_scale: 2.0,
            focal_length: 1.0,
        }
    }

    pub fn render(&self, image_size: Size) -> Bitmap {
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
                let color = self.bg_color(&ray);
                bitmap.set(i, j, color)
            }
        }
        eprint!("\n");

        bitmap
    }

    fn bg_color(&self, ray: &Ray) -> Color {
        let unit_direction = (ray.direction).as_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        // Color::new(1.0, 1.0, 1.0).lerp(t, Color::new(0.5, 0.7, 1.0))
        let rv = Color::new(1.0, 1.0, 1.0).lerp(t, Color::new(0.5, 0.7, 1.0));
        // Color::new(1.0, 1.0, 1.0).lerp(t, Color::new(0.0, 0.0, 0.0))
        rv
    }

}


fn clear_line() -> &'static str {
    "\x1B[2K\r"
}
