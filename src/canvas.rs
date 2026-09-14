use crate::{camera::ScreenPosition, color::Color};

/// A 3-dimensional drawing canvas using a Cartesian coordinate system.
pub struct Canvas {
    /// Width of `Canvas` in pixels.
    width: usize,
    /// Height of `Canvas` in pixels.
    height: usize,
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
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        let size = width
            .checked_mul(height)
            .ok_or("Canvas dimensions are too large".to_string())?;

        Ok(Self {
            width,
            height,
            color_buffer: vec![Color::BLACK.to_u32(); size],
            depth_buffer: vec![1.0; size],
        })
    }

    /// Returns the width of `Canvas` in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of `Canvas` in pixels.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an immutable reference to `color_buffer`.
    pub fn pixels(&self) -> &[u32] {
        &self.color_buffer
    }

    /// Clears `color_buffer` by setting every pixel to `color`.
    pub fn clear(&mut self, color: Color) {
        self.color_buffer.fill(color.to_u32());
    }

    /// Resets `depth_buffer` by setting every value to 1.0.
    pub fn clear_depth(&mut self) {
        self.depth_buffer.fill(1.0);
    }

    /// Translates Cartesian coordinates to framebuffer index.
    ///
    /// Returns `None` if outside the Canvas bounds.
    fn buffer_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let x = x.cast_unsigned();
        let y = y.cast_unsigned();

        Some((self.height - 1_usize - y as usize) * self.width + x as usize)
    }

    /// Sets the pixel at `(x, y)` to the given `color`.
    ///
    /// Pixels outside the Canvas bounds are discarded.
    #[expect(clippy::indexing_slicing, reason = "Bounds are checked manually")]
    pub fn set_pixel(&mut self, x: f32, y: f32, depth: f32, color: Color) {
        if let Some(index) = self.buffer_index(x.round() as i32, y.round() as i32)
            && depth <= self.depth_buffer[index]
        {
            self.depth_buffer[index] = depth;
            self.color_buffer[index] = color.to_u32();
        }
    }

    #[expect(clippy::indexing_slicing, reason = "Bounds are checked manually")]
    pub fn set_pixel_i32(&mut self, x: i32, y: i32, depth: f32, color: Color) {
        if let Some(index) = self.buffer_index(x, y)
            && depth <= self.depth_buffer[index]
        {
            self.depth_buffer[index] = depth;
            self.color_buffer[index] = color.to_u32();
        }
    }

    /// Draws a line from `start` to `end`.
    pub fn draw_line(&mut self, start: ScreenPosition, end: ScreenPosition, color: Color) {
        let mut x = start.x;
        let mut y = start.y;

        let dx = (end.x - x).abs();
        let x_step = if x < end.x { 1.0 } else { -1.0 };

        let dy = -(end.y - y).abs();
        let y_step = if y < end.y { 1.0 } else { -1.0 };

        let max_step = dx.max(dy.abs()) as f32;

        let dz = if max_step == 0.0 {
            0.0
        } else {
            (end.depth - start.depth) / max_step
        };
        let mut z = start.x;

        let mut err = dx + dy;

        loop {
            self.set_pixel(x, y, z, color);

            if (x_step > 0.0 && x >= end.x || x_step < 0.0 && x <= end.x)
                && (y_step > 0.0 && y >= end.y || y_step < 0.0 && y <= end.y)
            {
                break;
            }
            if 2.0 * err >= dy {
                err += dy;
                x += x_step;
            }
            if 2.0 * err <= dx {
                err += dx;
                y += y_step;
            }
            z += dz;
        }
    }

    /// Draws an outline of a triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle(
        &mut self,
        _a: ScreenPosition,
        _b: ScreenPosition,
        _c: ScreenPosition,
        _color: Color,
    ) {
    }

    /// Draws a filled triangle with vertices `a`, `b` and `c`.
    pub fn fill_triangle(
        &mut self,
        _a: ScreenPosition,
        _b: ScreenPosition,
        _c: ScreenPosition,
        _color: Color,
    ) {
    }
}
