/// A 2-dimensional drawing canvas using a Cartesian coordinate system.
pub struct Canvas {
    width: u32,
    height: u32,
    color_buffer: Vec<u32>,
    depth_buffer: Vec<f32>,
}

impl Canvas {
    /// Creates a new Canvas with the given dimensions.
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

    pub fn pixels(&self) -> &[u32] {
        &self.color_buffer
    }

    /// Clears `color_buffer` by filling every pixel with `color`.
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

    /// Plots a single pixel at the given coordinates `(x, y)`.
    /// Coordinates are rounded to the nearest integer.
    /// Pixels outside the Canvas bounds are discarded.
    pub fn set_pixel(&mut self, x: f32, y: f32, color: u32) {
        if let Some(i) = self.index(x, y) {
            self.color_buffer[i] = color;
        }
    }

    /// Converts cartesian coordinates `(x, y)` to a buffer index.
    /// Returns `None` if out of bounds.
    fn index(&self, x: f32, y: f32) -> Option<usize> {
        let offset_width = self.width as i32 / 2 + x.round() as i32;
        let offset_height = self.height as i32 / 2 - y.round() as i32;

        if offset_width < 0
            || offset_width >= self.width as i32
            || offset_height < 0
            || offset_height >= self.height as i32
        {
            return None;
        }

        Some((offset_height * self.width as i32 + offset_width) as usize)
    }

    /// Draws a line between two points `(x1, y1)` and `(x2, y2)`.
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: u32) {
        let mut x1 = x1.round() as i32;
        let mut y1 = y1.round() as i32;
        let x2 = x2.round() as i32;
        let y2 = y2.round() as i32;

        let dx = (x2 - x1).abs();
        let x_step = if x1 < x2 { 1 } else { -1 };

        let dy = -(y2 - y1).abs();
        let y_step = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.set_pixel(x1 as f32, y1 as f32, color);

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

    /// Draws an outline of a circle with the given radius, centered at `(x1, y1)`.
    /// Based on [Alois Zingl's implementation](https://zingl.github.io/bresenham.html).
    pub fn draw_circle(&mut self, x1: f32, y1: f32, radius: f32, color: u32) {
        let x1 = x1.round() as i32;
        let y1 = y1.round() as i32;
        let radius = radius.round() as i32;

        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;

        while x <= 0 {
            self.set_pixel((x1 - x) as f32, (y1 + y) as f32, color);
            self.set_pixel((x1 - y) as f32, (y1 - x) as f32, color);
            self.set_pixel((x1 + x) as f32, (y1 - y) as f32, color);
            self.set_pixel((x1 + y) as f32, (y1 + x) as f32, color);

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

    /// Draws a filled circle with the given radius, centered at `(x1, y1)`.
    pub fn draw_circle_filled(&mut self, x1: f32, y1: f32, radius: f32, color: u32) {
        let x1 = x1.round() as i32;
        let y1 = y1.round() as i32;
        let radius = radius.round() as i32;

        let mut x = -radius;
        let mut y = 0;
        let mut err = 2 - 2 * radius;

        while x <= 0 {
            for x_curr in x1 - x..=x1 + x {
                self.set_pixel(x_curr as f32, (y1 + y) as f32, color);
                self.set_pixel(x_curr as f32, (y1 - y) as f32, color);
            }
            for x_curr in x1 - y..=x1 + y {
                self.set_pixel(x_curr as f32, (y1 + x) as f32, color);
                self.set_pixel(x_curr as f32, (y1 - x) as f32, color);
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
