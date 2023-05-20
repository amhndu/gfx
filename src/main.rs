use ray::{
    bitmap::PPM,
    render::Raytracer,
    types::*,
};

fn main() -> anyhow::Result<()> {
    let size = Size::new(400, 400 * 8 / 16 /* 16:8 ratio */);

    let bitmap = Raytracer::new().render(size);
    PPM.save(&bitmap, &mut std::io::stdout())?;

    Result::Ok(())
}
