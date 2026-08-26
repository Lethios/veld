use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    #[must_use]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;
    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[must_use]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[must_use]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[must_use]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[must_use]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg<Vec2> for Vec2 {
    type Output = Self;
    #[must_use]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
