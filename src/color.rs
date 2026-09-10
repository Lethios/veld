use std::ops::{Add, Mul, Sub};

/// A color represented by red, green, blue, alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red channel value in the range [0.0, 1.0].
    pub r: f32,
    /// Green channel value in the range [0.0, 1.0].
    pub g: f32,
    /// Blue channel value in the range [0.0, 1.0].
    pub b: f32,
    /// Alpha channel value in the range [0.0, 1.0].
    pub a: f32,
}

impl Color {
    /// Constants for common colors.
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// Creates a new `Color` with the given `rgba` values.
    ///
    /// Values are clamped to the interval [0.0, 1.0].
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = r.clamp(0.0, 1.0);
        let g = g.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);
        let a = a.clamp(0.0, 1.0);

        Self { r, g, b, a }
    }

    /// Creates a `Color` with `rgba` components between 0 and 255.
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let a = a as f32 / 255.0;

        Self { r, g, b, a }
    }

    /// Creates a `Color` from a u32 (`0xRRGGBB`) with alpha set to 1.0.
    pub const fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        let a = 255_u8;

        Self::from_rgba(r, g, b, a)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }
}

#[expect(
    clippy::cast_sign_loss,
    reason = "Values are clamped to the interval [0.0, 1.0]"
)]
impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        let r = (value.r * 255.0).round() as u32;
        let g = (value.g * 255.0).round() as u32;
        let b = (value.b * 255.0).round() as u32;
        let a = (value.a * 255.0).round() as u32;

        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs, self.a)
    }
}
