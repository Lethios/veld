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

/// A camera defined by `position`, `yaw` and `pitch`.
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

    /// Moves `Camera` along its local forward axis.
    ///
    /// Negative `distance` moves backwards.
    pub fn move_forward(&mut self, distance: f32) {
        self.position = self.position + self.local_forward() * distance;
    }

    /// Moves `Camera` along its local right axis.
    ///
    /// Negative `distance` moves left.
    pub fn move_right(&mut self, distance: f32) {
        self.position = self.position + self.local_right() * distance;
    }

    /// Moves `Camera` along its local up axis.
    ///
    /// Negative `distance` moves down.
    pub fn move_up(&mut self, distance: f32) {
        self.position = self.position + self.local_up() * distance;
    }

    /// Rotates `Camera` horizontally to the right.
    ///
    /// Negative `angle` rotates left.
    pub fn rotate_yaw(&mut self, angle: f32) {
        self.yaw += angle;
    }

    /// Rotates `Camera` vertically upward.
    ///
    /// Negative `angle` rotates downward.
    ///
    /// `pitch` is clamped to keep the view matrix stable.
    pub fn rotate_pitch(&mut self, angle: f32) {
        self.pitch = (self.pitch + angle).clamp(
            -std::f32::consts::FRAC_PI_2 + 0.001,
            std::f32::consts::FRAC_2_PI - 0.001,
        );
    }

    /// Returns the `Camera`'s view matrix.
    ///
    /// The view matrix transforms coordinates from world space into view space.
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(
            self.position,
            self.position + self.local_forward(),
            self.local_up(),
        )
    }

    /// Returns the `Camera`'s projection matrix.
    ///
    /// The projection matrix transforms coordinates from view space into clip space.
    pub fn projection_matrix(&self, width: u32, height: u32) -> Mat4 {
        let aspect = width as f32 / height as f32;

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

    /// Transforms a point from world space into camera (view) space.
    pub fn world_to_camera(&self, world: Vec3) -> Vec3 {
        let res = self.view_matrix() * world.to_homogeneous();

        Vec3::new(res.x, res.y, res.z)
    }

    /// Transforms a point from camera space into clip space.
    pub fn camera_to_clip(&self, camera: Vec3, width: u32, height: u32) -> Vec4 {
        self.projection_matrix(width, height) * camera.to_homogeneous()
    }

    /// Transforms a point from clip space into normalized device coordinates (NDC).
    ///
    /// Returns `None` if the point is behind the camera or outside the NDC bounds.
    pub fn clip_to_ndc(&self, clip: Vec4) -> Option<Vec3> {
        if clip.w <= 0.0 {
            return None;
        }

        let ndc = Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);

        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 || ndc.z < -1.0 || ndc.z > 1.0
        {
            return None;
        }

        Some(ndc)
    }

    /// Transforms normalized device coordinates (NDC) into screen space coordinates.
    pub fn ndc_to_screen(&self, ndc: Vec3, width: u32, height: u32) -> Vec3 {
        Vec3::new(
            ndc.x * width as f32 / 2.0,
            ndc.y * height as f32 / 2.0,
            ndc.z,
        )
    }

    /// Runs the full pipeline of transforming world space position into screen space coordinates.
    pub fn project(&self, world: Vec3, width: u32, height: u32) -> Option<Vec3> {
        let camera = self.world_to_camera(world);
        let clip = self.camera_to_clip(camera, width, height);
        let ndc = self.clip_to_ndc(clip)?;
        let screen = self.ndc_to_screen(ndc, width, height);

        Some(screen)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, -10.0),
            0.0,
            0.0,
            Projection::Perspective {
                fov: std::f32::consts::FRAC_PI_3,
            },
            0.1,
            1000.0,
        )
    }
}
