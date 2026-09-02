use crate::{Mat4, Vec3, Vec4};

/// The projection mode used by `Camera`.
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
        self.local_right().cross(self.local_forward())
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

    pub fn project(&self, vec: Vec3, screen_width: f32, screen_height: f32) -> Option<Vec3> {
        let view_proj_matrix = self.view_projection(screen_width / screen_height);

        // Convert to clip space
        let clip = view_proj_matrix * Vec4::new(vec.x, vec.y, vec.z, 1.0);

        // Reject if behind camera
        // TODO: implement full frustum clipping
        if clip.w <= 0.0 {
            return None;
        }

        // Convert to NDC
        let ndc = Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);

        // Reject if outside view frustum
        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 || ndc.z < -1.0 || ndc.z > 1.0
        {
            return None;
        }

        // Convert to screen coordinates
        Some(Vec3::new(
            ndc.x * screen_width / 2.0,
            ndc.y * screen_height / 2.0,
            ndc.z,
        ))
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
