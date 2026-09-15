use crate::{Color, Vec3};

/// A vertex in screen space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct ScreenVertex {
    pub position: Vec3,
    pub color: Color,
}

impl ScreenVertex {
    /// Create a new `ScreenVertex`.
    pub fn new(position: Vec3, color: Color) -> Self {
        Self { position, color }
    }
}
