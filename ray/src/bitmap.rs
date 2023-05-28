use crate::vec::*;
use anyhow::Result;

// A Bitmap image.
//
// +ve y
// ^
// |
// |
// 0 --------> +ve x
#[derive(Debug)]
pub struct Bitmap {
    size: Size,

    // pixels are stored in a row-major order
    data: Vec<Color>,
}

trait Serializer {
    fn save(bitmap: &Bitmap, target: &mut impl std::io::Write) -> Result<()>;
}

pub struct PPM;

impl Bitmap {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            data: vec![Color::ZERO; size.area() as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn get(&self, x: u32, y: u32) -> Color {
        self.data[Self::index(self.size.width, self.size.height, x, y)]
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        self.data[Self::index(self.size.width, self.size.height, x, y)] = color;
    }

    #[inline(always)]
    fn index(width: u32, height: u32, x: u32, mut y: u32) -> usize {
        y = height - y - 1;
        (y * width + x) as usize
    }
}

impl PPM {
    pub fn save(self, bitmap: &Bitmap, target: &mut impl std::io::Write) -> Result<()> {
        write!(
            target,
            "P3\n{} {}\n255\n",
            bitmap.size.width, bitmap.size.height
        )?;
        for y in (0..bitmap.size.height).rev() {
            for x in 0..bitmap.size.width {
                let color = bitmap.get(x, y);
                write!(
                    target,
                    "{} {} {}\n",
                    Self::to_256(color.r()),
                    Self::to_256(color.g()),
                    Self::to_256(color.b())
                )?;
            }
        }

        Result::Ok(())
    }

    #[inline]
    fn to_256(v: f64) -> i64 {
        (256.0 * v.clamp(0.0, 0.999)).floor() as i64
    }
}
