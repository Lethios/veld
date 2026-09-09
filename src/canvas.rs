use crate::{Mat2, Vec2, Vec3, camera::ScreenPosition, color::Color};

/// A 3-dimensional drawing canvas using a Cartesian coordinate system.
pub struct Canvas {
    /// Width of `Canvas` in pixels.
    width: u32,
    /// Height of `Canvas` in pixels.
    height: u32,
    /// Pixels are stored in `0xAARRGGBB` format.
    color_buffer: Vec<u32>,
    /// Positive values represent points farther away.
    depth_buffer: Vec<f32>,
}

impl Canvas {
    /// Creates a new `Canvas`.
    ///
    /// `color_buffer` is initialized with all pixels set to black, while
    /// `depth_buffer` is initialized with all values set to 1.0.
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let size = (width as usize)
            .checked_mul(height as usize)
            .ok_or("Canvas dimensions are too large".to_string())?;

        Ok(Self {
            width,
            height,
            color_buffer: vec![Color::BLACK.into(); size],
            depth_buffer: vec![1.0; size],
        })
    }

    /// Returns the width of `Canvas` in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of `Canvas` in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns an immutable reference to `color_buffer`.
    pub fn pixels(&self) -> &[u32] {
        &self.color_buffer
    }

    /// Clears `color_buffer` by setting every pixel to `color`.
    pub fn clear(&mut self, color: Color) {
        self.color_buffer.fill(color.into());
    }

    /// Resets `depth_buffer` by setting every value to 1.0.
    pub fn clear_depth(&mut self) {
        self.depth_buffer.fill(1.0);
    }

    /// Translates Cartesian coordinates to framebuffer index.
    ///
    /// Returns `None` if outside the Canvas bounds.
    fn buffer_index(&self, x: u32, y: u32) -> Option<usize> {
        Some(((self.height - 1 - y) * self.width + x) as usize)
    }

    /// Sets the pixel at `(x, y)` to the given `color`.
    ///
    /// Pixels outside the Canvas bounds are discarded.
    #[expect(clippy::indexing_slicing, reason = "Bounds are checked manually")]
    fn set_pixel(&mut self, x: f32, y: f32, depth: f32, color: Color) {
        if let Some(index) = self.buffer_index(x.round() as u32, y.round() as u32)
            && depth <= self.depth_buffer[index]
        {
            self.depth_buffer[index] = depth;
            self.color_buffer[index] = color.into();
        }
    }

    /// Draws a line from `start` to `end`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_line(&mut self, start: ScreenPosition, end: ScreenPosition, color: Color) {}

    /// Draws an outline of a circle with the given `radius`, centered at `center`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_circle(&mut self, center: ScreenPosition, radius: f32, color: Color) {}

    /// Draws a filled circle with the given `radius`, centered at `center`.
    pub fn draw_circle_filled(&mut self, center: ScreenPosition, radius: f32, color: Color) {}

    /// Draws an outline of a triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle(
        &mut self,
        a: ScreenPosition,
        b: ScreenPosition,
        c: ScreenPosition,
        color: Color,
    ) {
    }

    /// Draws a filled triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle_filled(
        &mut self,
        a: ScreenPosition,
        b: ScreenPosition,
        c: ScreenPosition,
        color: Color,
    ) {
    }
}
