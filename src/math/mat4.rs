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

    /// Returns the determinant of `self`.
    pub fn determinant(&self) -> f32 {
        let xx = self.x_axis.x;
        let xy = self.x_axis.y;
        let xz = self.x_axis.z;
        let xw = self.x_axis.w;

        let yx = self.y_axis.x;
        let yy = self.y_axis.y;
        let yz = self.y_axis.z;
        let yw = self.y_axis.w;

        let zx = self.z_axis.x;
        let zy = self.z_axis.y;
        let zz = self.z_axis.z;
        let zw = self.z_axis.w;

        let wx = self.w_axis.x;
        let wy = self.w_axis.y;
        let wz = self.w_axis.z;
        let ww = self.w_axis.w;

        xx * (yy * (zz * ww - zw * wz) - zy * (yz * ww - yw * wz) + wy * (yz * zw - zz * yw))
            - yx * (xy * (zz * ww - zw * wz) - zy * (xz * ww - xw * wz) + wy * (xz * zw - zz * xw))
            + zx * (xy * (yz * ww - yw * wz) - yy * (xz * ww - xw * wz) + wy * (xz * yw - yz * xw))
            - wx * (xy * (yz * zw - zz * yw) - yy * (xz * zw - zz * xw) + zy * (xz * yw - yz * xw))
    }

    /// Returns the inverse of `self`.
    pub fn inverse(&self) -> Option<Self> {
        let m00 = self.x_axis.x;
        let m01 = self.y_axis.x;
        let m02 = self.z_axis.x;
        let m03 = self.w_axis.x;

        let m10 = self.x_axis.y;
        let m11 = self.y_axis.y;
        let m12 = self.z_axis.y;
        let m13 = self.w_axis.y;

        let m20 = self.x_axis.z;
        let m21 = self.y_axis.z;
        let m22 = self.z_axis.z;
        let m23 = self.w_axis.z;

        let m30 = self.x_axis.w;
        let m31 = self.y_axis.w;
        let m32 = self.z_axis.w;
        let m33 = self.w_axis.w;

        let a2323 = m22 * m33 - m23 * m32;
        let a1323 = m21 * m33 - m23 * m31;
        let a1223 = m21 * m32 - m22 * m31;
        let a0323 = m20 * m33 - m23 * m30;
        let a0223 = m20 * m32 - m22 * m30;
        let a0123 = m20 * m31 - m21 * m30;

        let a2313 = m12 * m33 - m13 * m32;
        let a1313 = m11 * m33 - m13 * m31;
        let a1213 = m11 * m32 - m12 * m31;

        let a2312 = m12 * m23 - m13 * m22;
        let a1312 = m11 * m23 - m13 * m21;
        let a1212 = m11 * m22 - m12 * m21;

        let a0313 = m10 * m33 - m13 * m30;
        let a0213 = m10 * m32 - m12 * m30;
        let a0312 = m10 * m23 - m13 * m20;
        let a0212 = m10 * m22 - m12 * m20;
        let a0113 = m10 * m31 - m11 * m30;
        let a0112 = m10 * m21 - m11 * m20;

        let det = m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
            - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
            + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
            - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123);

        if det == 0.0 {
            return None;
        }

        let det = 1.0 / det;

        // These are the rows of the conventional inverse.
        //
        // We transpose them into columns when constructing our
        // column-major Matrix4.
        Some(Self {
            x_axis: Vec4::new(
                det * (m11 * a2323 - m12 * a1323 + m13 * a1223),
                det * -(m10 * a2323 - m12 * a0323 + m13 * a0223),
                det * (m10 * a1323 - m11 * a0323 + m13 * a0123),
                det * -(m10 * a1223 - m11 * a0223 + m12 * a0123),
            ),

            y_axis: Vec4::new(
                det * -(m01 * a2323 - m02 * a1323 + m03 * a1223),
                det * (m00 * a2323 - m02 * a0323 + m03 * a0223),
                det * -(m00 * a1323 - m01 * a0323 + m03 * a0123),
                det * (m00 * a1223 - m01 * a0223 + m02 * a0123),
            ),

            z_axis: Vec4::new(
                det * (m01 * a2313 - m02 * a1313 + m03 * a1213),
                det * -(m00 * a2313 - m02 * a0313 + m03 * a0213),
                det * (m00 * a1313 - m01 * a0313 + m03 * a0113),
                det * -(m00 * a1312 - m01 * a0312 + m03 * a0112),
            ),

            w_axis: Vec4::new(
                det * -(m01 * a2312 - m02 * a1312 + m03 * a1212),
                det * (m00 * a2312 - m02 * a0312 + m03 * a0212),
                det * -(m00 * a1312 - m01 * a0312 + m02 * a0112),
                det * (m00 * a1212 - m01 * a0212 + m02 * a0112),
            ),
        })
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
    pub fn view(position: Vec3, target: Vec3, world_up: Vec3) -> Self {
        let forward = (position - target).normalize();
        let right = world_up.cross(forward).normalize();
        let up = forward.cross(right).normalize();

        Mat4::new(
            Vec4::new(right.x, up.x, forward.x, 0.0),
            Vec4::new(right.y, up.y, forward.y, 0.0),
            Vec4::new(right.z, up.z, forward.z, 0.0),
            Vec4::new(
                -right.dot(position),
                -up.dot(position),
                -forward.dot(position),
                1.0,
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
