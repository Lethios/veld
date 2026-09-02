use crate::math::{Vec3, Vec4};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 4x4 matrix. Uses the column-major order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Mat4 {
    /// Returns a `Mat4` with all elements set to 0.0.
    pub const ZERO: Self = Self::new(
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 0.0),
    );

    /// Returns a `Mat4` identity matrix.
    pub const IDENTITY: Self = Self::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );

    /// Creates a new `Mat4`.
    pub const fn new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    /// Returns the transpose of `self`.
    pub fn transpose(&self) -> Self {
        Self::new(
            Vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            Vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            Vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        )
    }

    /// Returns a scaling matrix.
    pub fn scale(v: Vec3) -> Self {
        Self::new(
            Vec4::new(v.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, v.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, v.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Returns a translation matrix.
    pub fn translate(v: Vec3) -> Self {
        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(v.x, v.y, v.z, 1.0),
        )
    }

    /// Returns a rotation matrix around the X-axis.
    ///
    /// `angle` is in radians.
    pub fn rotate_x(angle: f32) -> Self {
        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, angle.cos(), angle.sin(), 0.0),
            Vec4::new(0.0, -angle.sin(), angle.cos(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Returns a rotation matrix around the Y-axis.
    ///
    /// `angle` is in radians.
    pub fn rotate_y(angle: f32) -> Self {
        Self::new(
            Vec4::new(angle.cos(), 0.0, -angle.sin(), 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(angle.sin(), 0.0, angle.cos(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Returns a rotation matrix around the Z-axis.
    ///
    /// `angle` is in radians.
    pub fn rotate_z(angle: f32) -> Self {
        Self::new(
            Vec4::new(angle.cos(), angle.sin(), 0.0, 0.0),
            Vec4::new(-angle.sin(), angle.cos(), 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Returns a view matrix.
    pub fn look_at(position: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - position).normalize();
        let right = forward.cross(up).normalize();
        let up = forward.cross(right).normalize();

        Mat4::new(
            Vec4::new(right.x, up.x, -forward.x, 0.0),
            Vec4::new(right.y, up.y, -forward.y, 0.0),
            Vec4::new(right.z, up.z, -forward.z, 0.0),
            Vec4::new(
                -right.dot(position),
                -up.dot(position),
                forward.dot(position),
                0.0,
            ),
        )
    }

    /// Returns a perspective projection matrix.
    ///
    /// Uses OpenGL NDC conventions.
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_half_fov: f32 = (fov / 2.0).tan();
        Self::new(
            Vec4::new(1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0 / tan_half_fov, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -(far + near) / (far - near), -1.0),
            Vec4::new(0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0),
        )
    }

    /// Returns an orthographic projection matrix.
    ///
    /// Uses OpenGL NDC conventions.
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self::new(
            Vec4::new(2.0 / (right - left), 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 / (top - bottom), 0.0, 0.0),
            Vec4::new(0.0, 0.0, -2.0 / (far - near), 0.0),
            Vec4::new(
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.0,
            ),
        )
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, rhs: Mat4) -> Self::Output {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
            w_axis: self.w_axis + rhs.w_axis,
        }
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, rhs: Mat4) -> Self::Output {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
            w_axis: self.w_axis - rhs.w_axis,
        }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Mat4) -> Self::Output {
        Self {
            x_axis: self * rhs.x_axis,
            y_axis: self * rhs.y_axis,
            z_axis: self * rhs.z_axis,
            w_axis: self * rhs.w_axis,
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x_axis.x * rhs.x
                + self.y_axis.x * rhs.y
                + self.z_axis.x * rhs.z
                + self.w_axis.x * rhs.w,
            self.x_axis.y * rhs.x
                + self.y_axis.y * rhs.y
                + self.z_axis.y * rhs.z
                + self.w_axis.y * rhs.w,
            self.x_axis.z * rhs.x
                + self.y_axis.z * rhs.y
                + self.z_axis.z * rhs.z
                + self.w_axis.z * rhs.w,
            self.x_axis.w * rhs.x
                + self.y_axis.w * rhs.y
                + self.z_axis.w * rhs.z
                + self.w_axis.w * rhs.w,
        )
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x_axis: self.x_axis * rhs,
            y_axis: self.y_axis * rhs,
            z_axis: self.z_axis * rhs,
            w_axis: self.w_axis * rhs,
        }
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x_axis: self.x_axis / rhs,
            y_axis: self.y_axis / rhs,
            z_axis: self.z_axis / rhs,
            w_axis: self.w_axis / rhs,
        }
    }
}

impl Neg for Mat4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x_axis: -self.x_axis,
            y_axis: -self.y_axis,
            z_axis: -self.z_axis,
            w_axis: -self.w_axis,
        }
    }
}
