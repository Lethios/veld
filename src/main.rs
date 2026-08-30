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
    let mut canvas = Canvas::new(200, 200);

    canvas.set_pixel(0.0, 0.0, 0xFFFF0000);
    canvas.draw_circle_filled(Vec2::ZERO, 25.0, 0xFF0000FF);
    canvas.draw_line(Vec2::new(0.0, 25.0), Vec2::new(-21.65, -12.5), 0xFF00FF00);
    canvas.draw_line(
        Vec2::new(-21.65, -12.5),
        Vec2::new(21.65, -12.5),
        0xFF00FF00,
    );
    canvas.draw_line(Vec2::new(21.65, -12.5), Vec2::new(0.0, 25.0), 0xFF00FF00);
    canvas.draw_circle(Vec2::ZERO, 50.0, 0xFFFF0000);

    save_ppm(canvas.pixels(), 200, 200, "out.ppm");
}
