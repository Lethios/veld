use crate::Vec2;

/// A 2-dimensional drawing canvas using a Cartesian coordinate system.
///
/// The `color_buffer` stores pixels in `0xAARRGGBB` format (alpha will be ignored by the display backend).
pub struct Canvas {
    width: u32,
    height: u32,
    color_buffer: Vec<u32>,
    depth_buffer: Vec<f32>,
}

impl Canvas {
    /// Creates a new Canvas with the given dimensions.
    ///
    /// `color_buffer` is initialized with all pixels set to black, while
    /// `depth_buffer` is initialized with all values set to infinity.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![0x00000000; (width as usize).checked_mul(height as usize).unwrap()],
            depth_buffer: vec![f32::INFINITY; (width * height) as usize],
        }
    }

    /// Returns the width of the Canvas in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the Canvas in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a reference to `color_buffer`.
    pub fn pixels(&self) -> &[u32] {
        &self.color_buffer
    }

    /// Clears `color_buffer` by setting every pixel to `color`.
    pub fn clear(&mut self, color: u32) {
        for elem in self.color_buffer.iter_mut() {
            *elem = color;
        }
    }

    /// Resets `depth_buffer` by setting every value to infinity.
    pub fn clear_depth(&mut self) {
        for elem in self.depth_buffer.iter_mut() {
            *elem = f32::INFINITY;
        }
    }

    /// Sets the pixel at `(x, y)` to the given `color`.
    ///
    /// Coordinates are rounded to the nearest integer.
    /// Pixels outside the Canvas bounds are discarded.
    pub fn set_pixel(&mut self, x: f32, y: f32, color: u32) {
        let x = x.round() as i32;
        let y = y.round() as i32;

        let offset_width = self.width as i32 / 2 + x;
        let offset_height = self.height as i32 / 2 - y;

        let index = offset_height * self.width as i32 + offset_width;

        if let Some(value) = self.color_buffer.get_mut(index as usize) {
            *value = color;
        }
    }

    /// Sets the pixel at `(x, y)` to the given `color`.
    ///
    /// Pixels outside the Canvas bounds are discarded.
    fn set_pixel_i32(&mut self, x: i32, y: i32, color: u32) {
        let offset_width = self.width as i32 / 2 + x;
        let offset_height = self.height as i32 / 2 - y;

        let index = offset_height * self.width as i32 + offset_width;

        if let Some(value) = self.color_buffer.get_mut(index as usize) {
            *value = color;
        }
    }

    /// Draws a line from `start` to `end`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_line(&mut self, start: Vec2, end: Vec2, color: u32) {
        let mut x1 = start.x.round() as i32;
        let mut y1 = start.y.round() as i32;
        let x2 = end.x.round() as i32;
        let y2 = end.y.round() as i32;

        let dx = (x2 - x1).abs();
        let x_step = if x1 < x2 { 1 } else { -1 };

        let dy = -(y2 - y1).abs();
        let y_step = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.set_pixel_i32(x1, y1, color);

            if x1 == x2 && y1 == y2 {
                break;
            }
            if 2 * err >= dy {
                err += dy;
                x1 += x_step;
            }
            if 2 * err <= dx {
                err += dx;
                y1 += y_step;
            }
        }
    }

    /// Draws an outline of a circle with the given `radius`, centered at `center`.
    ///
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_circle(&mut self, center: Vec2, radius: f32, color: u32) {
        let x1 = center.x.round() as i32;
        let y1 = center.y.round() as i32;
        let radius = radius.round() as i32;

        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;

        while x <= 0 {
            self.set_pixel_i32(x1 - x, y1 + y, color);
            self.set_pixel_i32(x1 - y, y1 - x, color);
            self.set_pixel_i32(x1 + x, y1 - y, color);
            self.set_pixel_i32(x1 + y, y1 + x, color);

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
    pub fn draw_circle_filled(&mut self, center: Vec2, radius: f32, color: u32) {
        let x1 = center.x.round() as i32;
        let y1 = center.y.round() as i32;
        let radius = radius.round() as i32;

        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;

        while x <= 0 {
            for x_curr in x1 - x..=x1 + x {
                self.set_pixel_i32(x_curr, y1 + y, color);
                self.set_pixel_i32(x_curr, y1 - y, color);
            }
            for x_curr in x1 - y..=x1 + y {
                self.set_pixel_i32(x_curr, y1 + x, color);
                self.set_pixel_i32(x_curr, y1 - x, color);
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
}
