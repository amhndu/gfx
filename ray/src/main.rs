use std::rc::Rc;

use rand::{random, Rng};
use ray::{
    bitmap::PPM,
    hittable::*,
    material::*,
    render::{Config, Raytracer},
    vec::*,
};

fn random_scene() -> World {
    let mut world = World::new();

    let ground_mat = ApproxLambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000, 0),
        1000.0,
        ground_mat.clone(),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                let rng = &mut rand::thread_rng();
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random::<Color>() * random::<Color>();
                    ApproxLambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(rng, 0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // glass
                    Dielectric::new(1.5)
                };
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0, 1, 0), 1.0, material1));

    let material2 = ApproxLambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4, 1, 0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4, 1, 0), 1.0, material3));

    return world;
}

fn main() -> anyhow::Result<()> {
    let size = Size::from_aspect_ratio(1200, 3.0 / 2.0);

    let world = random_scene();

    let lookfrom = Point3::new(13, 2, 3);
    let lookto = Point3::new(0, 0, 0);
    let config = Config {
        lookfrom,
        lookto,
        vertical_fov: 20f64.to_radians(),
        aperture: 0.1,
        focus_dist: 10.0,
        samples_per_pixel: 500,
        ..Default::default()
    };
    let raytracer = Raytracer::new(config, Rc::new(world));

    let bitmap = raytracer.render(size);
    PPM.save(&bitmap, &mut std::io::stdout())?;

    Result::Ok(())
}
