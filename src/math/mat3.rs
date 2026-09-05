use crate::math::{Vec2, Vec3};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 3x3 matrix. Uses the column-major order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Mat3 {
    /// Returns a `Mat3` with all elements set to 0.0.
    pub const ZERO: Self = Self::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    /// Returns a `Mat3` identity matrix.
    pub const IDENTITY: Self = Self::new(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    /// Creates a new `Mat3`.
    pub const fn new(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Returns the transpose of `self`.
    pub fn transpose(&self) -> Self {
        Self::new(
            Vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            Vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            Vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
        )
    }

    /// Returns the determinant of `self`.
    pub fn determinant(&self) -> f32 {
        self.x_axis.x * (self.y_axis.y * self.z_axis.z - self.z_axis.y * self.y_axis.z)
            - self.y_axis.x * (self.x_axis.y * self.z_axis.z - self.z_axis.y * self.x_axis.z)
            + self.z_axis.x * (self.x_axis.y * self.y_axis.z - self.y_axis.y * self.x_axis.z)
    }

    /// Returns the inverse of `self`.
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();

        if det == 0.0 {
            return None;
        }

        let (a, d, g) = (self.x_axis.x, self.x_axis.y, self.x_axis.z);
        let (b, e, h) = (self.y_axis.x, self.y_axis.y, self.y_axis.z);
        let (c, f, i) = (self.z_axis.x, self.z_axis.y, self.z_axis.z);

        Some(
            Self::new(
                Vec3::new(e * i - f * h, -(d * i - f * g), d * h - e * g),
                Vec3::new(-(b * i - c * h), a * i - c * g, -(a * h - b * g)),
                Vec3::new(b * f - c * e, -(a * f - c * d), a * e - b * d),
            ) / det,
        )
    }

    /// Returns a scaling matrix.
    pub fn scale(v: Vec2) -> Self {
        Self::new(
            Vec3::new(v.x, 0.0, 0.0),
            Vec3::new(0.0, v.y, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }

    /// Returns a translation matrix.
    pub fn translate(v: Vec2) -> Self {
        Self::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(v.x, v.y, 1.0),
        )
    }

    /// Returns a rotation matrix around the X-axis.
    ///
    /// `angle` is in radians.
    pub fn rotate_x(angle: f32) -> Self {
        Self::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, angle.cos(), angle.sin()),
            Vec3::new(0.0, -angle.sin(), angle.cos()),
        )
    }

    /// Returns a rotation matrix around the Y-axis.
    ///
    /// `angle` is in radians.
    pub fn rotate_y(angle: f32) -> Self {
        Self::new(
            Vec3::new(angle.cos(), 0.0, -angle.sin()),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(angle.sin(), 0.0, angle.cos()),
        )
    }

    /// Returns a rotation matrix around the Z-axis.
    ///
    /// `angle` is in radians.
    pub fn rotate_z(angle: f32) -> Self {
        Self::new(
            Vec3::new(angle.cos(), angle.sin(), 0.0),
            Vec3::new(-angle.sin(), angle.cos(), 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }
}

impl Add for Mat3 {
    type Output = Self;

    fn add(self, rhs: Mat3) -> Self::Output {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
        }
    }
}

impl Sub for Mat3 {
    type Output = Self;

    fn sub(self, rhs: Mat3) -> Self::Output {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
        }
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Mat3) -> Self::Output {
        Self {
            x_axis: self * rhs.x_axis,
            y_axis: self * rhs.y_axis,
            z_axis: self * rhs.z_axis,
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.z_axis.x * rhs.z,
            self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.z_axis.y * rhs.z,
            self.x_axis.z * rhs.x + self.y_axis.z * rhs.y + self.z_axis.z * rhs.z,
        )
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x_axis: self.x_axis * rhs,
            y_axis: self.y_axis * rhs,
            z_axis: self.z_axis * rhs,
        }
    }
}

impl Div<f32> for Mat3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x_axis: self.x_axis / rhs,
            y_axis: self.y_axis / rhs,
            z_axis: self.z_axis / rhs,
        }
    }
}

impl Neg for Mat3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x_axis: -self.x_axis,
            y_axis: -self.y_axis,
            z_axis: -self.z_axis,
        }
    }
}
