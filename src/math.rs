use std::{fmt::Display, ops};

use rand::Rng;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn zero() -> Self {
        Vec3([0.0, 0.0, 0.0])
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3([e0, e1, e2])
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Vec3::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }

    pub fn random_in_range(rng: &mut impl Rng, min: f64, max: f64) -> Self {
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
        loop {
            let p = Vec3::random_in_range(rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut impl Rng) -> Self {
        Vec3::random_in_unit_sphere(rng).unit()
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0.iter().map(|e| e * e).sum()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0.iter().zip(rhs.0.iter()).map(|(l, r)| l * r).sum()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ])
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.iter().all(|comp| comp.abs() < s)
    }

    pub fn reflect(self, other: Vec3) -> Vec3 {
        self - 2.0 * self.dot(other) * other
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let reciprical = 1.0 / rhs;
        self.0[0] *= reciprical;
        self.0[1] *= reciprical;
        self.0[2] *= reciprical;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}
