use crate::{Mat2, Vec2, Vec3, color::Color};

/// A 3-dimensional drawing canvas using a Cartesian coordinate system.
pub struct Canvas {
    /// Width of `Canvas` in pixels.
    width: u32,
    /// Height of `Canvas` in pixels.
    height: u32,
    /// Pixels are stored in `0xAARRGGBB` format (alpha will be ignored by the display backend).
    color_buffer: Vec<u32>,
    /// Negative values represent points farther away.
    depth_buffer: Vec<f32>,
}

impl Canvas {
    /// Creates a new `Canvas`.
    ///
    /// `color_buffer` is initialized with all pixels set to black, while
    /// `depth_buffer` is initialized with all values set to 1.0.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![
                Color::BLACK.into();
                (width as usize).checked_mul(height as usize).unwrap()
            ],
            depth_buffer: vec![1.0; (width * height) as usize],
        }
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
        for elem in self.color_buffer.iter_mut() {
            *elem = color.into();
        }
    }

    /// Resets `depth_buffer` by setting every value to infinity.
    pub fn clear_depth(&mut self) {
        for elem in self.depth_buffer.iter_mut() {
            *elem = 1.0;
        }
    }

    /// Converts Cartesian coordinates to framebuffer index.
    ///
    /// Returns `None` if outside the Canvas bounds.
    fn buffer_index(&self, x: i32, y: i32) -> Option<usize> {
        let offset_width = self.width as i32 / 2 + x;
        let offset_height = self.height as i32 / 2 - y;

        // Bounds check
        if offset_width < 0
            || offset_width >= self.width as i32
            || offset_height < 0
            || offset_height >= self.height as i32
        {
            return None;
        }

        let offset_width = offset_width.cast_unsigned();
        let offset_height = offset_height.cast_unsigned();

        Some((offset_height * self.width + offset_width) as usize)
    }

    /// Sets the pixel at `(x, y)` to the given `color`.
    ///
    /// Coordinates are rounded to the nearest integer.
    /// Pixels outside the Canvas bounds are discarded.
    #[expect(clippy::indexing_slicing, reason = "Bounds are checked manually")]
    pub fn set_pixel(&mut self, x: f32, y: f32, z: f32, color: Color) {
        let x = x.round() as i32;
        let y = y.round() as i32;

        if let Some(index) = self.buffer_index(x, y)
            && z <= self.depth_buffer[index]
        {
            self.depth_buffer[index] = z;
            self.color_buffer[index] = color.into();
        }
    }

    /// Sets the pixel at `(x, y)` to the given `color`.
    ///
    /// Pixels outside the Canvas bounds are discarded.
    #[expect(clippy::indexing_slicing, reason = "Bounds are checked manually")]
    fn set_pixel_i32(&mut self, x: i32, y: i32, z: f32, color: Color) {
        if let Some(index) = self.buffer_index(x, y)
            && z <= self.depth_buffer[index]
        {
            self.depth_buffer[index] = z;
            self.color_buffer[index] = color.into();
        }
    }

    /// Draws a line from `start` to `end`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_line(&mut self, start: Vec3, end: Vec3, color: Color) {
        let mut x = start.x.round() as i32;
        let mut y = start.y.round() as i32;

        let x_end = end.x.round() as i32;
        let y_end = end.y.round() as i32;

        let dx = (x_end - x).abs();
        let x_step = if x < x_end { 1 } else { -1 };

        let dy = -(y_end - y).abs();
        let y_step = if y < y_end { 1 } else { -1 };

        let max_step = dx.max(dy.abs()) as f32;

        let dz = if max_step == 0.0 {
            0.0
        } else {
            (end.z - start.z) / max_step
        };
        let mut z = start.z - 0.001; // Offset z value by bias to prevent z-fighting

        let mut err = dx + dy;

        loop {
            self.set_pixel_i32(x, y, z, color);

            if (x_step > 0 && x >= x_end || x_step < 0 && x <= x_end)
                && (y_step > 0 && y >= y_end || y_step < 0 && y <= y_end)
            {
                break;
            }
            if 2 * err >= dy {
                err += dy;
                x += x_step;
            }
            if 2 * err <= dx {
                err += dx;
                y += y_step;
            }
            z += dz;
        }
    }

    /// Draws an outline of a circle with the given `radius`, centered at `center`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_circle(&mut self, center: Vec3, radius: f32, color: Color) {
        let x_cen = center.x.round() as i32;
        let y_cen = center.y.round() as i32;
        let z = center.z;
        let radius = radius.round() as i32;

        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;

        while x <= 0 {
            self.set_pixel_i32(x_cen - x, y_cen + y, z, color);
            self.set_pixel_i32(x_cen - y, y_cen - x, z, color);
            self.set_pixel_i32(x_cen + x, y_cen - y, z, color);
            self.set_pixel_i32(x_cen + y, y_cen + x, z, color);

            let prev_err = err;

            if prev_err <= y {
                y += 1;
                err += 2 * y + 1;
            }
            if (prev_err > x) || (err > y) {
                x += 1;
                err += 2 * x + 1;
            }
        }
    }

    /// Draws a filled circle with the given `radius`, centered at `center`.
    pub fn draw_circle_filled(&mut self, center: Vec3, radius: f32, color: Color) {
        let x_cen = center.x.round() as i32;
        let y_cen = center.y.round() as i32;
        let z = center.z;
        let radius = radius.round() as i32;

        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;

        while x <= 0 {
            for x_curr in x_cen - x..=x_cen + x {
                self.set_pixel_i32(x_curr, y_cen + y, z, color);
                self.set_pixel_i32(x_curr, y_cen - y, z, color);
            }
            for x_curr in x_cen - y..=x_cen + y {
                self.set_pixel_i32(x_curr, y_cen + x, z, color);
                self.set_pixel_i32(x_curr, y_cen - x, z, color);
            }

            let prev_err = err;

            if prev_err <= y {
                y += 1;
                err += 2 * y + 1;
            }
            if (prev_err > x) || (err > y) {
                x += 1;
                err += 2 * x + 1;
            }
        }
    }

    /// Draws an outline of a triangle with vertices `a`, `b` and `c`.
    pub fn draw_triangle(&mut self, a: Vec3, b: Vec3, c: Vec3, color: Color) {
        self.draw_line(a, b, color);
        self.draw_line(b, c, color);
        self.draw_line(c, a, color);
    }

    /// Draws a filled triangle with vertices `a`, `b` and `c`.
    ///
    /// Use counter-clockwise winding for front faces. Triangles with clockwise winding are culled.
    pub fn draw_triangle_filled(&mut self, a: Vec3, b: Vec3, c: Vec3, color: Color) {
        // Back-face culling
        let ab_edge = b - a;
        let ac_edge = c - a;
        if ab_edge.cross(ac_edge).z <= 0.0 {
            return;
        }

        // Coordinates of bounding box
        let top_left = Vec2::new(a.x.min(b.x).min(c.x), a.y.max(b.y).max(c.y));
        let bottom_right = Vec2::new(a.x.max(b.x).max(c.x), a.y.min(b.y).min(c.y));

        let inverse = match Mat2::new(
            Vec2::new(b.x - a.x, b.y - a.y),
            Vec2::new(c.x - a.x, c.y - a.y),
        )
        .inverse()
        {
            Some(val) => val,
            None => return,
        };

        // Test for each pixel in the bounding box
        for x in (top_left.x.round() as i32)..=(bottom_right.x.round() as i32) {
            for y in (bottom_right.y.round() as i32)..=(top_left.y.round() as i32) {
                let weights = inverse * Vec2::new(x as f32 - a.x, y as f32 - a.y);
                let weights = Vec3::new(weights.x, weights.y, 1.0 - weights.x - weights.y);

                if (weights.x >= -1e-5)
                    && (weights.y >= -1e-5)
                    && (weights.x + weights.y <= 1.0 + 1e-5)
                {
                    let z = weights.x * a.z + weights.y * b.z + weights.z * c.z;
                    self.set_pixel_i32(x, y, z, color);
                }
            }
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new(800, 600)
    }
}
