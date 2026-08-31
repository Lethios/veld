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
    let mut canvas = Canvas::new(320, 250);

    canvas.set_pixel(7.0, 45.0, 0xFF0000FF);
    canvas.set_pixel(35.0, 100.0, 0xFF0000FF);
    canvas.set_pixel(45.0, 60.0, 0xFF0000FF);
    canvas.set_pixel(120.0, 35.0, 0xFF0000FF);
    canvas.set_pixel(90.0, 5.0, 0xFF0000FF);
    canvas.set_pixel(45.0, 110.0, 0xFF0000FF);
    canvas.set_pixel(115.0, 83.0, 0xFF0000FF);
    canvas.set_pixel(80.0, 90.0, 0xFF0000FF);
    canvas.set_pixel(85.0, 120.0, 0xFF0000FF);

    canvas.draw_triangle_filled(
        Vec2::new(7.0, 45.0),
        Vec2::new(35.0, 100.0),
        Vec2::new(45.0, 60.0),
        0xFFFF0000,
    );
    canvas.draw_triangle_filled(
        Vec2::new(120.0, 35.0),
        Vec2::new(90.0, 5.0),
        Vec2::new(45.0, 110.0),
        0xFFFFFFFF,
    );
    canvas.draw_triangle_filled(
        Vec2::new(115.0, 83.0),
        Vec2::new(80.0, 90.0),
        Vec2::new(85.0, 120.0),
        0xFF00FF00,
    );

    save_ppm(canvas.pixels(), 320, 250, "out.ppm");
}
