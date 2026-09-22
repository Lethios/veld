use crate::{Color, Vec3, Vec4};

/// A vertex in world space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct WorldVertex {
    pub position: Vec3,
    pub color: Color,
}

impl WorldVertex {
    /// Create a new `WorldVertex`
    pub fn new(position: Vec3, color: Color) -> Self {
        Self { position, color }
    }
}

const PLANES: [fn(Vec4) -> f32; 6] = [
    |point| point.w + point.x,
    |point| point.w - point.x,
    |point| point.w + point.y,
    |point| point.w - point.y,
    |point| point.w + point.z,
    |point| point.w - point.z,
];

/// A vertex in clip space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct ClipVertex {
    pub position: Vec4,
    pub color: Color,
}

impl ClipVertex {
    pub fn new(position: Vec4, color: Color) -> Self {
        Self { position, color }
    }

    fn lerp(&self, next: Self, t: f32) -> Self {
        Self::new(
            self.position + (next.position - self.position) * t,
            self.color + (next.color - self.color) * t,
        )
    }

    pub fn within_plane(&self) -> bool {
        PLANES.iter().all(|plane| plane(self.position) >= 0.0)
    }

    pub fn clip_polygon(vertices: &[Self]) -> Vec<Self> {
        let mut polygon_verts = vertices.to_vec();

        if vertices.iter().all(|v| v.within_plane()) {
            return polygon_verts;
        }

        for plane in PLANES {
            polygon_verts = Self::clip_plane(&polygon_verts, plane);

            if polygon_verts.is_empty() {
                break;
            }
        }

        polygon_verts
    }

    fn clip_plane(polygon_verts: &[Self], plane: fn(Vec4) -> f32) -> Vec<Self> {
        let mut res = Vec::with_capacity(polygon_verts.len() + 1);

        let Some(&last) = polygon_verts.last() else {
            return res;
        };

        let mut prev = last;
        let mut prev_dist = plane(prev.position);

        for &curr in polygon_verts {
            let curr_dist = plane(curr.position);

            if (prev_dist >= 0.0) != (curr_dist >= 0.0) {
                let t = prev_dist / (prev_dist - curr_dist);
                res.push(prev.lerp(curr, t));
            }

            if curr_dist >= 0.0 {
                res.push(curr);
            }

            prev = curr;
            prev_dist = curr_dist;
        }

        res
    }

    pub fn clip_line(start: Self, end: Self) -> Option<(Self, Self)> {
        let (mut t0, mut t1) = (0.0_f32, 1.0_f32);

        for plane in PLANES {
            let start_dist = plane(start.position);
            let end_dist = plane(end.position);

            if start_dist < 0.0 && end_dist < 0.0 {
                return None;
            }

            let t = start_dist / (start_dist - end_dist);

            if start_dist < 0.0 {
                t0 = t0.max(t);
            } else if end_dist < 0.0 {
                t1 = t1.min(t);
            }
        }

        if t0 > t1 {
            return None;
        }

        Some((start.lerp(end, t0), start.lerp(end, t1)))
    }
}

/// A vertex in screen space with attributes.
#[derive(Debug, Clone, Copy)]
pub struct ScreenVertex {
    pub position: Vec3,
    pub color: Color,
    pub inv_w: f32,
}

impl ScreenVertex {
    /// Create a new `ScreenVertex`.
    pub fn new(position: Vec3, color: Color, inv_w: f32) -> Self {
        Self {
            position,
            color,
            inv_w,
        }
    }
}
