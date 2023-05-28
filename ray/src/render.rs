use std::rc::Rc;

use crate::bitmap::Bitmap;
use crate::hittable::*;
use crate::material::*;
use crate::ray::Ray;
use crate::vec::*;
use rand::random;

pub struct Raytracer {
    config: Config,
    world: Rc<dyn Hittable>,
}

pub struct Config {
    // camera config
    pub lookfrom: Point3,
    pub lookto: Point3,
    pub vup: Vec3,
    pub vertical_fov: f64,
    pub viewport_scale: f64,
    // camera - focus
    pub focus_dist: f64,
    pub aperture: f64,
    // renderer config
    pub samples_per_pixel: u32,
    pub bounce_limit: u32,
}

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_raidus: f64,
    u: Vec3,
    v: Vec3,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lookto: Point3::new(0.0, 0.0, -1.0),
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vertical_fov: 120.0f64.to_radians(),
            viewport_scale: 2.0,
            samples_per_pixel: 100,
            bounce_limit: 50,
            focus_dist: 1.0,
            aperture: 1.0,
        }
    }
}

impl Raytracer {
    pub fn new(config: Config, world: Rc<dyn Hittable>) -> Self {
        Self { world, config }
    }

    pub fn render(self, image_size: Size) -> Bitmap {
        let camera = Camera::new(image_size, &self.config);
        let mut bitmap = Bitmap::new(image_size);
        let render_start = std::time::Instant::now();

        for j in 0..bitmap.height() {
            eprint!(
                "{}{}/{} scanlines done. ({}%) [{:.1?} elapsed]",
                clear_line(),
                j,
                bitmap.height() - 1,
                100 * j / (bitmap.height() - 1),
                render_start.elapsed(),
            );
            for i in 0..bitmap.width() {
                // perform anti-aliasing by randomized super-sampling
                let color: Color = (0..self.config.samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        let u = (i as f64 + random::<f64>()) / (bitmap.width() as f64 - 1.0);
                        let v = (j as f64 + random::<f64>()) / (bitmap.height() as f64 - 1.0);

                        let ray = camera.ray_at(u, v);
                        self.project(&ray, self.config.bounce_limit)
                    })
                    .sum();
                bitmap.set(i, j, self.emit_color(color))
            }
        }
        eprint!("\n");

        bitmap
    }

    fn emit_color(&self, mut color: Color) -> Color {
        let scale = 1.0 / self.config.samples_per_pixel as f64;
        // normalize after super-sampling
        color *= scale;
        // gamma correction
        color.sqrt()
    }

    fn project(&self, ray: &Ray, bounce_limit: u32) -> Color {
        if bounce_limit <= 0 {
            return Color::ZERO;
        }

        if let Some(hit) = self.world.hit(ray, 0.001, std::f64::INFINITY) {
            // assume a matte surface: diffuse to a random direction
            return match hit.material.scatter(ray, &hit.record) {
                ScatterResult::Scattered {
                    scattered,
                    attenuation,
                } => {
                    let color = self.project(&scattered, bounce_limit - 1);
                    return attenuation * color;
                }
                ScatterResult::Absorbed => Color::ZERO,
            };
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

impl Camera {
    fn new(image_size: Size, config: &Config) -> Self {
        let h = (config.vertical_fov / 2.0).tan();
        let viewport_height = config.viewport_scale * h;
        let viewport_width = image_size.aspect_ratio() * viewport_height;

        let origin = config.lookfrom;
        assert!(config.vup.length_sq() == 1.0);
        let w = (config.lookfrom - config.lookto).as_unit();
        let u = config.vup.cross(w).as_unit();
        let v = w.cross(u);
        let horizontal = u * config.focus_dist * viewport_width;
        let vertical = v * config.focus_dist * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * config.focus_dist;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_raidus: config.aperture / 2.0,
            u,
            v,
        }
    }

    fn ray_at(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_raidus * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
