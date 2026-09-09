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
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

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

    /// Draws an outline of a circle with the given `radius`, centered at `center`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_circle(&mut self, center: ScreenPosition, radius: f32, color: Color) {
        let radius = radius.round();

        let mut x = -radius;
        let mut y = 0.0;
        let mut err = 2.0 - 2.0 * radius;

        while x <= 0.0 {
            self.set_pixel(center.x - x, center.y + y, center.depth, color);
            self.set_pixel(center.x - y, center.y - x, center.depth, color);
            self.set_pixel(center.x + x, center.y - y, center.depth, color);
            self.set_pixel(center.x + y, center.y + x, center.depth, color);

            let prev_err = err;

            if prev_err <= y {
                y += 1.0;
                err += 2.0 * y + 1.0;
            }
            if (prev_err > x) || (err > y) {
                x += 1.0;
                err += 2.0 * x + 1.0;
            }
        }
    }

    /// Draws a filled circle with the given `radius`, centered at `center`.
    pub fn draw_circle_filled(&mut self, center: ScreenPosition, radius: f32, color: Color) {
        let radius = radius.round();

        let mut x = -radius;
        let mut y = 0.0;
        let mut err = 2.0 - 2.0 * radius;

        while x <= 0.0 {
            for x_curr in (center.x - x).round() as u32..=(center.x + x).round() as u32 {
                self.set_pixel(x_curr as f32, center.y + y, center.depth, color);
                self.set_pixel(x_curr as f32, center.y - y, center.depth, color);
            }
            for x_curr in (center.x - y).round() as u32..=(center.x + y).round() as u32 {
                self.set_pixel(x_curr as f32, center.y + x, center.depth, color);
                self.set_pixel(x_curr as f32, center.y - x, center.depth, color);
            }

            let prev_err = err;

            if prev_err <= y {
                y += 1.0;
                err += 2.0 * y + 1.0;
            }
            if (prev_err > x) || (err > y) {
                x += 1.0;
                err += 2.0 * x + 1.0;
            }
        }
    }

    /// Draws an outline of a triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle(
        &mut self,
        a: ScreenPosition,
        b: ScreenPosition,
        c: ScreenPosition,
        color: Color,
    ) {
        self.draw_line(a, b, color);
        self.draw_line(b, c, color);
        self.draw_line(c, a, color);
    }

    /// Draws a filled triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle_filled(
        &mut self,
        a: ScreenPosition,
        b: ScreenPosition,
        c: ScreenPosition,
        color: Color,
    ) {
        // Coordinates of bounding box
        let top_left = (
            a.x.min(b.x).min(c.x).round() as u32,
            a.y.max(b.y).max(c.y).round() as u32,
        );
        let bottom_right = (
            a.x.max(b.x).max(c.x).round() as u32,
            a.y.min(b.y).min(c.y).round() as u32,
        );

        let inverse = match Mat2::new(
            Vec2::new((b.x - a.x) as f32, (b.y - a.y) as f32),
            Vec2::new((c.x - a.x) as f32, (c.y - a.y) as f32),
        )
        .inverse()
        {
            Some(val) => val,
            None => return,
        };

        // Test for each pixel in the bounding box
        for x in top_left.0..=bottom_right.0 {
            for y in bottom_right.1..=top_left.1 {
                let weights = inverse * Vec2::new((x as f32 - a.x) as f32, (y as f32 - a.y) as f32);
                let weights = Vec3::new(weights.x, weights.y, 1.0 - weights.x - weights.y);

                if (weights.x >= -1e-5)
                    && (weights.y >= -1e-5)
                    && (weights.x + weights.y <= 1.0 + 1e-5)
                {
                    let z = weights.x * a.depth + weights.y * b.depth + weights.z * c.depth;
                    self.set_pixel(x as f32, y as f32, z, color);
                }
            }
        }
    }
}
