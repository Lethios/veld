pub mod canvas;
pub mod math;

use veld::canvas::Canvas;
use veld::math::*;

fn save_ppm(pixels: &[u32], width: u32, height: u32, path: &str) {
    let mut out = format!("P6\n{width} {height}\n255\n").into_bytes();
    for &p in pixels {
        out.push(((p >> 16) & 0xFF) as u8);
        out.push(((p >> 8) & 0xFF) as u8);
        out.push((p & 0xFF) as u8);
    }
    std::fs::write(path, out).unwrap();
}

fn main() {
    let canvas = Canvas::new(400, 400);

    save_ppm(canvas.pixels(), 400, 400, "out.ppm");
}
