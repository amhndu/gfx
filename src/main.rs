use ray::{
    bitmap::PPM,
    render::Raytracer,
    types::*,
    hittable::*,
};

fn main() -> anyhow::Result<()> {
    let size = Size::new(400, 400 * 8 / 16 /* 16:8 ratio */);

    let mut raytracer = Raytracer::new();
    raytracer.add(Box::new(Sphere { centre: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 }));

    let bitmap = raytracer.render(size);
    PPM.save(&bitmap, &mut std::io::stdout())?;

    Result::Ok(())
}
