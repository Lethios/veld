pub struct Canvas {
    width: u32,
    height: u32,
    color_buffer: Vec<u32>,
    depth_buffer: Vec<f32>,
}

impl Canvas {
    /// Creates a new Canvas with the given dimensions.
    /// `color_buffer` is initialized with all pixels set to black.
    /// `depth_buffer` is initialized with all values set to infinity.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![0u32; (width * height) as usize],
            depth_buffer: vec![f32::INFINITY; (width * height) as usize],
        }
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

    /// Plots a single pixel at the given cartesian coordinates (x, y).
    /// Coordinates are rounded to the nearest integer.
    /// (0, 0) is the center of Canvas. Out-of-bound coordinates are discarded.
    fn set_pixel(&mut self, x: f32, y: f32, color: u32) {
        let Some(i) = self.index(x.round() as i32, y.round() as i32) else {
            return;
        };
        self.color_buffer[i] = color;
    }

    /// Converts cartesian coordinates (x, y) to a buffer index.
    /// Returns None if out of bounds.
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let offset_width = (self.width as i32 / 2) + x;
        let offset_height = (self.height as i32 / 2) - y;

        if offset_width < 0
            || offset_width >= self.width as i32
            || offset_height < 0
            || offset_height >= self.height as i32
        {
            return None;
        }

        Some((offset_height * self.width as i32 + offset_width) as usize)
    }

    /// Draws a line between two cartesian coordinates (x1, y1) and (x2, y2)
    /// Uses Bresenham's line algorithm
    /// Coordinates are rounded to nearest integer before rasterization.
    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: u32) {
        let x1 = x1.round() as i32;
        let y1 = y1.round() as i32;
        let x2 = x2.round() as i32;
        let y2 = y2.round() as i32;

        if (y2 - y1).abs() < (x2 - x1).abs() {
            if x1 > x2 {
                self.draw_line_low(x2, y2, x1, y1, color);
            } else {
                self.draw_line_low(x1, y1, x2, y2, color);
            }
        } else if y1 > y2 {
            self.draw_line_high(x2, y2, x1, y1, color);
        } else {
            self.draw_line_high(x1, y1, x2, y2, color);
        }
    }

    /// Handles lines where |dx| > |dy|.
    fn draw_line_low(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let dx = x2 - x1;
        let mut dy = y2 - y1;
        let mut y_step = 1;

        if dy < 0 {
            y_step = -1;
            dy = -dy;
        }

        let mut err = (2 * dy) - dx;
        let mut y_curr = y1;

        for x_curr in x1..x2 {
            self.set_pixel(x_curr as f32, y_curr as f32, color);

            if err > 0 {
                y_curr += y_step;
                err += 2 * (dy - dx);
            } else {
                err += 2 * dy;
            }
        }
    }

    /// Handles lines where |dy| >= |dx|.
    fn draw_line_high(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let mut dx = x2 - x1;
        let dy = y2 - y1;
        let mut x_step = 1;

        if dx < 0 {
            x_step = -1;
            dx = -dx;
        }

        let mut err = (2 * dx) - dy;
        let mut x_curr = x1;

        for y_curr in y1..y2 {
            self.set_pixel(x_curr as f32, y_curr as f32, color);

            if err > 0 {
                x_curr += x_step;
                err += 2 * (dx - dy);
            } else {
                err += 2 * dx;
            }
        }
    }
}
