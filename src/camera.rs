use crate::{Mat4, Vec3};

/// The projection mode used by `Camera`.
#[derive(Clone, Copy)]
pub enum Projection {
    /// Refer to [Orthographic Projection](https://en.wikipedia.org/wiki/Orthographic_projection).
    ///
    /// `scale` is the half-height of the view volume.
    Orthographic { scale: f32 },
    /// Refer to [Perspective Projection](https://en.wikipedia.org/wiki/Perspective_(graphical)).
    ///
    /// `fov` is the vertical field of view in radians.
    Perspective { fov: f32 },
}

/// A camera defined by position, yaw and pitch.
pub struct Camera {
    /// World space position of `Camera`.
    pub position: Vec3,
    /// Horizontal rotation in radians.
    pub yaw: f32,
    /// Vertical rotation in radians.
    pub pitch: f32,
    /// Projection mode.
    pub projection: Projection,
    /// Near clipping plane distance.
    pub near: f32,
    /// Far clipping plane distance.
    pub far: f32,
}

impl Camera {
    /// Creates a new `Camera`.
    pub fn new(
        position: Vec3,
        yaw: f32,
        pitch: f32,
        projection: Projection,
        near: f32,
        far: f32,
    ) -> Self {
        Camera {
            position,
            yaw,
            pitch,
            projection,
            near,
            far,
        }
    }

    /// Returns the `Camera`'s forward direction.
    pub fn local_forward(&self) -> Vec3 {
        Vec3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
    }

    /// Returns the `Camera`'s right direction.
    pub fn local_right(&self) -> Vec3 {
        Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin())
    }

    /// Returns the `Camera`'s up direction.
    pub fn local_up(&self) -> Vec3 {
        self.local_forward().cross(self.local_right())
    }

    /// Returns the matrix that transforms world space to camera space.
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(
            self.position,
            self.position + self.local_forward(),
            self.local_up(),
        )
    }

    /// Returns the projection matrix for the given `aspect` ratio.
    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        match self.projection {
            Projection::Orthographic { scale } => Mat4::orthographic(
                -(scale * aspect),
                scale * aspect,
                -scale,
                scale,
                self.near,
                self.far,
            ),
            Projection::Perspective { fov } => Mat4::perspective(fov, aspect, self.near, self.far),
        }
    }

    /// Returns the product of `projection_matrix` and `view_matrix`.
    pub fn view_projection(&self, aspect: f32) -> Mat4 {
        self.projection_matrix(aspect) * self.view_matrix()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, -10.0),
            0.0,
            0.0,
            Projection::Perspective {
                fov: 45.0_f32.to_radians(),
            },
            0.01,
            10000.0,
        )
    }
}
