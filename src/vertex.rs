use crate::{Color, ScreenPosition};

/// A vertex in screen space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: ScreenPosition,
    pub color: Color,
}

impl Vertex {
    /// Create a new `Vertex`.
    pub fn new(position: ScreenPosition, color: Color) -> Self {
        Self { position, color }
    }
}
