use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rand::prelude::*;

/// A 2-dimensional column vector
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Vector of all zeros
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Creates a new vector
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the length of the vector squared
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product of 'self' and 'rhs'
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the normalised vector of 'self' of length 1.0
    ///
    /// All values must finite and not Zero
    ///
    /// Length might also be slightly off 1.0 due to rounding errors from floating point numbers
    pub fn normalise(self) -> Self {
        self / self.length()
    }

    /// Creates a vector with random values [0.0,1.0)
    pub fn random() -> Self {
        Self {
            x: rand::random(),
            y: rand::random(),
        }
    }

    /// Creates a vector with random values [0.0,1.0) from a rand::Rng instance
    pub fn random_with_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            x: rng.random(),
            y: rng.random(),
        }
    }

    /// Creates a vector with random values [min,max)
    pub fn random_range(min: f32, max: f32) -> Self {
        Self {
            x: rand::random_range(min..max),
            y: rand::random_range(min..max),
        }
    }

    /// Creates a vector with random values [min,max) from a rand::Rng instance
    pub fn random_range_with_rng<R: rand::Rng + ?Sized>(min: f32, max: f32, rng: &mut R) -> Self {
        Self {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
        }
    }

    /// Create a vector with random values [0.0,1.0) with length 1.0
    pub fn random_unit() -> Self {
        loop {
            let v = Self::random_range(-1.0, 1.0);
            let lensq = v.length_squared();
            if 0.0 < lensq && lensq <= 1.0 {
                return v.normalise();
            }
        }
    }

    /// Create a vector with random values [0.0,1.0) with length 1.0 from a rand::Rng instance
    pub fn random_unit_with_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        loop {
            let v = Self::random_range_with_rng(-1.0, 1.0, rng);
            let lensq = v.length_squared();
            if 0.0 < lensq && lensq <= 1.0 {
                return v.normalise();
            }
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
