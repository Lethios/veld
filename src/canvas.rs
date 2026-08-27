pub struct Canvas {
    width: u32,
    height: u32,
    color_buffer: Vec<u32>,
    depth_buffer: Vec<f32>,
}

impl Canvas {
    /// Creates a new Canvas.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_buffer: Vec::with_capacity((width * height) as usize),
            depth_buffer: Vec::with_capacity((width * height) as usize),
        }
    }

    pub fn clear(&mut self, color: u32) {
        for elem in self.color_buffer.iter_mut() {
            *elem = color;
        }
    }

    pub fn clear_depth(&mut self) {
        for elem in self.depth_buffer.iter_mut() {
            *elem = f32::INFINITY;
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        let i = self.index(x, y);

        self.color_buffer[i] = color;
    }

    fn index(&self, x: i32, y: i32) -> usize {
        let offset_width = (self.width as i32 / 2) + x;
        let offset_height = (self.height as i32 / 2) - y;

        (offset_width * self.width as i32 + offset_height) as usize
    }
}
