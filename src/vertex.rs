use crate::{Color, Vec3};

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
