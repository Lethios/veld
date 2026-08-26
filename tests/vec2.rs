use veld::Vec2;

const EPSILON: f32 = 1e-6;
fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn test_new() {
    let v = Vec2::new(1.0, 2.0);
    assert!(approx_eq(v.x, 1.0));
    assert!(approx_eq(v.y, 2.0));
}

#[test]
fn test_add() {
    let result = Vec2::new(1.0, 2.0) + Vec2::new(3.0, 4.0);
    assert!(approx_eq(result.x, 4.0));
    assert!(approx_eq(result.y, 6.0));
}

#[test]
fn test_sub() {
    let result = Vec2::new(5.0, 3.0) - Vec2::new(2.0, 1.0);
    assert!(approx_eq(result.x, 3.0));
    assert!(approx_eq(result.y, 2.0));
}

#[test]
fn test_mul_vec2() {
    let result = Vec2::new(2.0, 3.0) * Vec2::new(4.0, 5.0);
    assert!(approx_eq(result.x, 8.0));
    assert!(approx_eq(result.y, 15.0));
}

#[test]
fn test_mul_scalar() {
    let result = Vec2::new(2.0, 3.0) * 2.0;
    assert!(approx_eq(result.x, 4.0));
    assert!(approx_eq(result.y, 6.0));
}

#[test]
fn test_div_vec2() {
    let result = Vec2::new(8.0, 9.0) / Vec2::new(2.0, 3.0);
    assert!(approx_eq(result.x, 4.0));
    assert!(approx_eq(result.y, 3.0));
}

#[test]
fn test_div_scalar() {
    let result = Vec2::new(6.0, 4.0) / 2.0;
    assert!(approx_eq(result.x, 3.0));
    assert!(approx_eq(result.y, 2.0));
}

#[test]
fn test_neg() {
    let result = -Vec2::new(1.0, -2.0);
    assert!(approx_eq(result.x, -1.0));
    assert!(approx_eq(result.y, 2.0));
}

#[test]
fn test_dot() {
    let result = Vec2::new(1.0, 2.0).dot(Vec2::new(3.0, 4.0));
    assert!(approx_eq(result, 11.0));
}

#[test]
fn test_length() {
    let result = Vec2::new(3.0, 5.0).length();
    assert!(approx_eq(result, 5.83));
}

#[test]
fn test_length_squared() {
    let result = Vec2::new(3.0, 4.0).length_squared();
    assert!(approx_eq(result, 25.0));
}

#[test]
fn test_normalize() {
    let result = Vec2::new(3.0, 4.0).normalize();
    assert!(approx_eq(result.length(), 1.0));
}
