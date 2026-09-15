use crate::{Color, ScreenVertex};

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
    pub fn draw_pixel(&mut self, point: ScreenVertex) {
        let (x, y, depth) = (
            point.position.x.round() as i32,
            point.position.y.round() as i32,
            point.position.z,
        );

        self.set_pixel(x, y, depth, point.color);
    }

    /// Draws a line from `start` to `end`.
    pub fn draw_line(&mut self, start: ScreenVertex, end: ScreenVertex) {
        let (mut x1, mut y1, mut z1, mut c1) = (
            start.position.x,
            start.position.y,
            start.position.z,
            start.color,
        );
        let (mut x2, mut y2, mut z2, mut c2) =
            (end.position.x, end.position.y, end.position.z, end.color);

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
    pub fn draw_triangle(&mut self, a: ScreenVertex, b: ScreenVertex, c: ScreenVertex) {
        self.draw_line(a, b);
        self.draw_line(b, c);
        self.draw_line(c, a);
    }

    /// Draws a filled triangle with vertices `a`, `b` and `c`.
    pub fn fill_triangle(&mut self, a: ScreenVertex, b: ScreenVertex, c: ScreenVertex) {
        const FIXED_SHIFT: i32 = 12;
        const FIXED_SCALE: f32 = (1 << FIXED_SHIFT) as f32;
        let to_fixed = |v: f32| -> i32 { (v * FIXED_SCALE).round() as i32 };

        let (ax, ay, az) = (to_fixed(a.position.x), to_fixed(a.position.y), a.position.z);
        let (bx, by, bz) = (to_fixed(b.position.x), to_fixed(b.position.y), b.position.z);
        let (cx, cy, cz) = (to_fixed(c.position.x), to_fixed(c.position.y), c.position.z);

        let mask = (1 << FIXED_SHIFT) - 1;

        let (x_min, x_max) = (
            ax.min(bx).min(cx) >> FIXED_SHIFT,
            (ax.max(bx).max(cx) + mask) >> FIXED_SHIFT,
        );
        let (y_min, y_max) = (
            ay.min(by).min(cy) >> FIXED_SHIFT,
            (ay.max(by).max(cy) + mask) >> FIXED_SHIFT,
        );

        let determinant = |ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32| -> i64 {
            (bx - ax) as i64 * (cy - ay) as i64 - (cx - ax) as i64 * (by - ay) as i64
        };
        let is_top_left = |ax: i32, ay: i32, bx: i32, by: i32| -> bool {
            let x_edge = bx - ax;
            let y_edge = by - ay;

            (y_edge < 0) || (y_edge == 0 && x_edge < 0)
        };

        if determinant(ax, ay, bx, by, cx, cy) <= 0 {
            return;
        }

        let area = determinant(ax, ay, bx, by, cx, cy) as f32;

        let bias1 = if is_top_left(bx, by, cx, cy) {
            0_i64
        } else {
            -(1_i64 << FIXED_SHIFT)
        };
        let bias2 = if is_top_left(cx, cy, ax, ay) {
            0_i64
        } else {
            -(1_i64 << FIXED_SHIFT)
        };
        let bias3 = if is_top_left(ax, ay, bx, by) {
            0_i64
        } else {
            -(1_i64 << FIXED_SHIFT)
        };

        let (a1, b1) = (by - cy, cx - bx);
        let (a2, b2) = (cy - ay, ax - cx);
        let (a3, b3) = (ay - by, bx - ax);

        let x = (x_min << FIXED_SHIFT) + (1 << (FIXED_SHIFT - 1));
        let y = (y_min << FIXED_SHIFT) + (1 << (FIXED_SHIFT - 1));

        let mut w1_row = determinant(bx, by, cx, cy, x, y) + bias1;
        let mut w2_row = determinant(cx, cy, ax, ay, x, y) + bias2;
        let mut w3_row = determinant(ax, ay, bx, by, x, y) + bias3;

        for y_int in y_min..=y_max {
            let (mut w1, mut w2, mut w3) = (w1_row, w2_row, w3_row);

            for x_int in x_min..=x_max {
                if w1 >= 0 && w2 >= 0 && w3 >= 0 {
                    let wt1 = w1 as f32 / area;
                    let wt2 = w2 as f32 / area;
                    let wt3 = w3 as f32 / area;

                    let z = wt1 * az + wt2 * bz + wt3 * cz;
                    let color = a.color * wt1 + b.color * wt2 + c.color * wt3;
                    self.set_pixel(x_int, y_int, z, color);
                }

                w1 += (a1 as i64) << FIXED_SHIFT;
                w2 += (a2 as i64) << FIXED_SHIFT;
                w3 += (a3 as i64) << FIXED_SHIFT;
            }

            w1_row += (b1 as i64) << FIXED_SHIFT;
            w2_row += (b2 as i64) << FIXED_SHIFT;
            w3_row += (b3 as i64) << FIXED_SHIFT;
        }
    }
}
