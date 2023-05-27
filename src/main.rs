use std::rc::Rc;

use ray::{bitmap::PPM, hittable::*, material::*, render::Raytracer, vec::*};

fn main() -> anyhow::Result<()> {
    let size = Size::new(400, 400 * 9 / 16 /* 16:9 ratio */);

    let ground_mat = ApproxLambertian::new(Color::new(0.8, 0.8, 0.0));
    let center_mat = ApproxLambertian::new(Color::new(0.1, 0.2, 0.5));
    // let left_mat = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let left_mat = Dielectric::new(1.5);
    let right_mat = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let mut world = World::new();
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_mat.clone(),
    ));
    world.add(Rc::new(Sphere {
        centre: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: center_mat.clone(),
    }));
    world.add(Rc::new(Sphere {
        centre: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: left_mat.clone(),
    }));
    world.add(Rc::new(Sphere {
        centre: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: left_mat.clone(),
    }));
    world.add(Rc::new(Sphere {
        centre: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: right_mat.clone(),
    }));
    let raytracer = Raytracer::new(Rc::new(world));

    let bitmap = raytracer.render(size);
    PPM.save(&bitmap, &mut std::io::stdout())?;

    Result::Ok(())
}
