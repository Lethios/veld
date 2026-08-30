use veld::math::Vec3;

#[test]
fn test_zero() {
    let v = Vec3::ZERO;

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}

#[test]
fn test_one() {
    let v = Vec3::ONE;

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn test_new() {
    let v = Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_dot() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(-2.0, 4.0, -3.0);

    assert_eq!(a.dot(b), -3.0);
}

#[test]
fn test_length() {
    let v = Vec3::new(3.0, 0.0, 4.0);

    assert_eq!(v.length(), 5.0);
}

#[test]
fn test_length_squared() {
    let v = Vec3::new(3.0, 0.0, 4.0);

    assert_eq!(v.length_squared(), 25.0);
}

#[test]
fn test_normalize() {
    let v = Vec3::new(4.0, 3.0, 0.0).normalize();

    assert_eq!(v.x, 0.8);
    assert_eq!(v.y, 0.6);
    assert_eq!(v.z, 0.0);
}

#[test]
fn test_add() {
    let a = Vec3::new(1.0, -2.0, 3.5);
    let b = Vec3::new(-5.0, 2.0, 1.5);
    let v = a + b;

    assert_eq!(v.x, -4.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 5.0);
}

#[test]
fn test_sub() {
    let a = Vec3::new(1.0, -2.0, 3.5);
    let b = Vec3::new(-5.0, 2.0, 1.5);
    let v = a - b;

    assert_eq!(v.x, 6.0);
    assert_eq!(v.y, -4.0);
    assert_eq!(v.z, 2.0);
}

#[test]
fn test_mul_f32() {
    let v = Vec3::new(1.0, -2.0, 3.5) * 2.0;

    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, -4.0);
    assert_eq!(v.z, 7.0);
}

#[test]
fn test_div_f32() {
    let v = Vec3::new(1.0, -2.0, 3.5) / 2.0;

    assert_eq!(v.x, 0.5);
    assert_eq!(v.y, -1.0);
    assert_eq!(v.z, 1.75);
}

#[test]
fn test_neg() {
    let v = -Vec3::new(1.0, -2.0, 3.5);

    assert_eq!(v.x, -1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, -3.5);
}
