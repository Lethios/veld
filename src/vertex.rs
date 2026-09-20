use crate::{Color, Vec3, Vec4};

/// A vertex in world space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct WorldVertex {
    pub position: Vec3,
    pub color: Color,
}

impl WorldVertex {
    /// Create a new `WorldVertex`
    pub fn new(position: Vec3, color: Color) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClipVertex {
    pub position: Vec4,
    pub color: Color,
}

impl ClipVertex {
    pub fn new(position: Vec4, color: Color) -> Self {
        Self { position, color }
    }
}

/// A vertex in screen space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct ScreenVertex {
    pub position: Vec3,
    pub color: Color,
    pub inv_w: f32,
}

impl ScreenVertex {
    /// Create a new `ScreenVertex`.
    pub fn new(position: Vec3, color: Color, inv_w: f32) -> Self {
        Self {
            position,
            color,
            inv_w,
        }
    }
}
