use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(f64, f64, f64);

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);

    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self(x.into(), y.into(), z.into())
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }

    pub fn r(self) -> f64 {
        self.0
    }

    pub fn g(self) -> f64 {
        self.1
    }

    pub fn b(self) -> f64 {
        self.2
    }

    pub fn length_sq(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, normal: Self, refraction_index_ratio: f64) -> Self {
        let cosine = (-self).dot(normal).min(1.0);
        let r_out_perp = refraction_index_ratio * (self + cosine * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_sq()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }

    pub fn as_unit(self) -> Self {
        self / self.length()
    }

    pub fn lerp(self, t: f64, rhs: Self) -> Self {
        (1.0 - t) * self + (t * rhs)
    }

    pub fn is_near_zero(self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.0.abs() < EPSILON && self.1.abs() < EPSILON && self.2.abs() < EPSILON
    }

    pub fn random_range<R: rand::Rng + ?Sized>(rng: &mut R, range: std::ops::Range<f64>) -> Self {
        Vec3(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let point = Vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if point.length_sq() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().as_unit()
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let point = Self::random_range(&mut rng, -1.0..1.0);
            if point.length_sq() < 1.0 {
                return point;
            }
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut final_vec = Self::ZERO;
        for item in iter.into_iter() {
            final_vec += item;
        }
        final_vec
    }
}

impl rand::distributions::Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn from_aspect_ratio(width: u32, aspect_ratio: f64) -> Self {
        Self {
            width,
            height: (width as f64 / aspect_ratio) as u32,
        }
    }

    pub fn area(self) -> u32 {
        self.width * self.height
    }

    pub fn aspect_ratio(self) -> f64 {
        self.width as f64 / self.height as f64
    }
}
