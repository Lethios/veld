use crate::math::Vec2;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 2x2 matrix. Uses the column-major order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

impl Mat2 {
    /// Returns a `Mat2` with all elements set to 0.0.
    pub const ZERO: Self = Self::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    /// Returns a `Mat2` identity matrix.
    pub const IDENTITY: Self = Self::new(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0));

    /// Creates a new `Mat2`.
    pub const fn new(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self { x_axis, y_axis }
    }

    /// Returns the transpose of `self`.
    pub fn transpose(&self) -> Self {
        Self::new(
            Vec2::new(self.x_axis.x, self.y_axis.x),
            Vec2::new(self.x_axis.y, self.y_axis.y),
        )
    }

    /// Returns the determinant of `self`.
    pub fn determinant(&self) -> f32 {
        self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
    }

    /// Returns the inverse of `self`.
    pub fn inverse(&self) -> Option<Self> {
        let (a, c) = (self.x_axis.x, self.x_axis.y);
        let (b, d) = (self.y_axis.x, self.y_axis.y);

        if self.determinant() != 0.0 {
            return Some(Self::new(Vec2::new(d, -c), Vec2::new(-b, a)) / self.determinant());
        }

        None
    }
}

impl Add for Mat2 {
    type Output = Self;

    fn add(self, rhs: Mat2) -> Self::Output {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
        }
    }
}

impl Sub for Mat2 {
    type Output = Self;

    fn sub(self, rhs: Mat2) -> Self::Output {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
        }
    }
}

impl Mul for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Mat2) -> Self::Output {
        Self {
            x_axis: self * rhs.x_axis,
            y_axis: self * rhs.y_axis,
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y,
        )
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x_axis: self.x_axis * rhs,
            y_axis: self.y_axis * rhs,
        }
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x_axis: self.x_axis / rhs,
            y_axis: self.y_axis / rhs,
        }
    }
}

impl Neg for Mat2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x_axis: -self.x_axis,
            y_axis: -self.y_axis,
        }
    }
}
