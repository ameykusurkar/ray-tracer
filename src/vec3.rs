use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn map(self, f: impl Fn(f32) -> f32) -> Self {
        Vec3(f(self.0), f(self.1), f(self.2))
    }

    pub fn reduce(self, f: impl Fn(f32, f32) -> f32) -> f32 {
        f(f(self.0, self.1), self.2)
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn magnitude(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self * rhs).reduce(Add::add)
    }

    pub fn cross(self, rhs: Self) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = (2.0 * Self::random()) - 1.0;
            if v.dot(v) < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disc() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = 2.0 * Vec3(rng.gen(), rng.gen(), 0.0) - Vec3(1.0, 1.0, 0.0);
            if v.dot(v) < 1.0 {
                return v;
            }
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), Add::add)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Vec3 {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Vec3 {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
