use veld::math::Vec4;

#[test]
fn test_zero() {
    let v = Vec4::ZERO;

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
    assert_eq!(v.w, 0.0);
}

#[test]
fn test_one() {
    let v = Vec4::ONE;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
    assert_eq!(v.w, 1.0);
}

#[test]
fn test_new() {
    let v = Vec4::new(-1.0, 0.0, 2.0, 3.0);

    assert_eq!(v.x, -1.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 2.0);
    assert_eq!(v.w, 3.0);
}

#[test]
fn test_dot() {
    let a = Vec4::new(-1.0, 2.0, 4.0, 0.0);
    let b = Vec4::new(-2.0, 3.0, 4.0, 2.0);

    assert_eq!(a.dot(b), 24.0);
}

#[test]
fn test_length() {
    let v = Vec4::new(1.0, 2.0, 2.0, 4.0);

    assert_eq!(v.length(), 5.0);
}

#[test]
fn test_length_squared() {
    let v = Vec4::new(1.0, 2.0, 2.0, 4.0);

    assert_eq!(v.length_squared(), 25.0);
}

#[test]
fn test_normalize() {
    let v = Vec4::new(1.0, 2.0, 2.0, 4.0).normalize();

    assert_eq!(v.x, 0.2);
    assert_eq!(v.y, 0.4);
    assert_eq!(v.z, 0.4);
    assert_eq!(v.w, 0.8);
}

#[test]
fn test_add() {
    let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let b = Vec4::new(-1.0, -2.0, -3.0, -4.0);
    let v = a + b;

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
    assert_eq!(v.w, 0.0);
}

#[test]
fn test_sub() {
    let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let b = Vec4::new(-1.0, -2.0, -3.0, -4.0);
    let v = a - b;

    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, 4.0);
    assert_eq!(v.z, 6.0);
    assert_eq!(v.w, 8.0);
}

#[test]
fn test_mul_f32() {
    let v = Vec4::new(-3.0, 0.0, 2.0, 1.5) * 2.0;

    assert_eq!(v.x, -6.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 4.0);
    assert_eq!(v.w, 3.0);
}

#[test]
fn test_div_f32() {
    let v = Vec4::new(2.0, 1.0, 4.0, 0.0) / 2.0;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 0.5);
    assert_eq!(v.z, 2.0);
    assert_eq!(v.w, 0.0);
}

#[test]
fn test_neg() {
    let v = -Vec4::new(-1.0, 3.5, 0.0, -2.5);

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, -3.5);
    assert_eq!(v.z, 0.0);
    assert_eq!(v.w, 2.5);
}
