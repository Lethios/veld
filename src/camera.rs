use crate::Vec3;

pub enum Projection {
    Perspective,
    Orthographic,
}

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub projection: Projection,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        projection: Projection,
        near: f32,
        far: f32,
    ) -> Self {
        Camera {
            position,
            target,
            up,
            fov,
            projection,
            near,
            far,
        }
    }

    pub fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, -10.0),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
            45.0_f32.to_radians(),
            Projection::Perspective,
            0.01,
            10000.0,
        )
    }
}
