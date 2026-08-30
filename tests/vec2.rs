use veld::math::Vec2;

#[test]
fn test_zero() {
    let v = Vec2::ZERO;

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
}

#[test]
fn test_one() {
    let v = Vec2::ONE;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
}

#[test]
fn test_new() {
    let v = Vec2::new(1.0, 2.0);

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
}

#[test]
fn test_dot() {
    let a = Vec2::new(0.0, 2.0);
    let b = Vec2::new(1.0, 4.0);

    assert_eq!(a.dot(b), 8.0);
}

#[test]
fn test_length() {
    let v = Vec2::new(3.0, 4.0);

    assert_eq!(v.length(), 5.0);
}

#[test]
fn test_length_squared() {
    let v = Vec2::new(3.0, 4.0);

    assert_eq!(v.length_squared(), 25.0);
}

#[test]
fn test_normalize() {
    let v = Vec2::new(3.0, 4.0).normalize();

    assert_eq!(v.x, 0.6);
    assert_eq!(v.y, 0.8);
}

#[test]
fn test_add() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let v = a + b;

    assert_eq!(v.x, 4.0);
    assert_eq!(v.y, 6.0);
}

#[test]
fn test_sub() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let v = a - b;

    assert_eq!(v.x, -2.0);
    assert_eq!(v.y, -2.0);
}

#[test]
fn test_mul_f32() {
    let v = Vec2::new(1.0, 2.0) * 1.5;

    assert_eq!(v.x, 1.5);
    assert_eq!(v.y, 3.0);
}

#[test]
fn test_div_f32() {
    let v = Vec2::new(1.5, 3.0) / 1.5;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
}

#[test]
fn test_neg() {
    let v = -Vec2::new(1.0, -2.0);

    assert_eq!(v.x, -1.0);
    assert_eq!(v.y, 2.0);
}
