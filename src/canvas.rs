use crate::{Color, Vertex};

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
            color_buffer: vec![Color::BLACK.to_u32_argb(); size],
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
    pub fn buffer(&self) -> &[u32] {
        &self.color_buffer
    }

    /// Clears `color_buffer` by setting every pixel to `color`.
    pub fn clear(&mut self, color: Color) {
        self.color_buffer.fill(color.to_u32_argb());
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
    fn set_pixel(&mut self, x: i32, y: i32, depth: f32, color: Color) {
        if let Some(index) = self.buffer_index(x, y)
            && depth <= self.depth_buffer[index]
        {
            self.depth_buffer[index] = depth;
            self.color_buffer[index] = color.to_u32_argb();
        }
    }

    /// Draws a single pixel at `point`.
    pub fn draw_pixel(&mut self, point: Vertex) {
        let (x, y, depth) = (
            point.position.x.round() as i32,
            point.position.y.round() as i32,
            point.position.depth,
        );

        self.set_pixel(x, y, depth, point.color);
    }

    /// Draws a line from `start` to `end`.
    pub fn draw_line(&mut self, start: Vertex, end: Vertex) {
        let (mut x1, mut y1, mut z1, mut c1) = (
            start.position.x,
            start.position.y,
            start.position.depth,
            start.color,
        );
        let (mut x2, mut y2, mut z2, mut c2) = (
            end.position.x,
            end.position.y,
            end.position.depth,
            end.color,
        );

        let steep = (x1 - x2).abs() < (y1 - y2).abs();

        if steep {
            std::mem::swap(&mut x1, &mut y1);
            std::mem::swap(&mut x2, &mut y2);
        }

        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
            std::mem::swap(&mut z1, &mut z2);
            std::mem::swap(&mut c1, &mut c2);
        }

        let dx = x2 - x1;
        let mut y = y1;

        for x in (x1.round() as i32)..=(x2.round() as i32) {
            let t = if dx == 0.0 { 0.0 } else { (x as f32 - x1) / dx };

            let z = z1 * (1.0 - t) + z2 * t;
            let color = c1 * (1.0 - t) + c2 * t;

            if steep {
                self.set_pixel(y.round() as i32, x, z, color);
            } else {
                self.set_pixel(x, y.round() as i32, z, color);
            }

            y += (y2 - y1) / dx;
        }
    }

    /// Draws an outline of a triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle(&mut self, _a: Vertex, _b: Vertex, _c: Vertex) {}

    /// Draws a filled triangle with vertices `a`, `b` and `c`.
    pub fn fill_triangle(&mut self, _a: Vertex, _b: Vertex, _c: Vertex) {}
}
