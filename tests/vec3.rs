use veld::Vec3;

const EPSILON: f32 = 1e-6;
fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn test_new() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(v.x, 1.0));
    assert!(approx_eq(v.y, 2.0));
    assert!(approx_eq(v.z, 3.0));
}

#[test]
fn test_add() {
    let result = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
    assert!(approx_eq(result.x, 5.0));
    assert!(approx_eq(result.y, 7.0));
    assert!(approx_eq(result.z, 9.0));
}

#[test]
fn test_sub() {
    let result = Vec3::new(5.0, 6.0, 7.0) - Vec3::new(2.0, 3.0, 4.0);
    assert!(approx_eq(result.x, 3.0));
    assert!(approx_eq(result.y, 3.0));
    assert!(approx_eq(result.z, 3.0));
}

#[test]
fn test_mul_vec3() {
    let result = Vec3::new(2.0, 3.0, 4.0) * Vec3::new(5.0, 6.0, 7.0);
    assert!(approx_eq(result.x, 10.0));
    assert!(approx_eq(result.y, 18.0));
    assert!(approx_eq(result.z, 28.0));
}

#[test]
fn test_mul_scalar() {
    let result = Vec3::new(2.0, 3.0, 4.0) * 2.0;
    assert!(approx_eq(result.x, 4.0));
    assert!(approx_eq(result.y, 6.0));
    assert!(approx_eq(result.z, 8.0));
}

#[test]
fn test_div_vec3() {
    let result = Vec3::new(8.0, 9.0, 10.0) / Vec3::new(2.0, 3.0, 5.0);
    assert!(approx_eq(result.x, 4.0));
    assert!(approx_eq(result.y, 3.0));
    assert!(approx_eq(result.z, 2.0));
}

#[test]
fn test_div_scalar() {
    let result = Vec3::new(6.0, 8.0, 10.0) / 2.0;
    assert!(approx_eq(result.x, 3.0));
    assert!(approx_eq(result.y, 4.0));
    assert!(approx_eq(result.z, 5.0));
}

#[test]
fn test_neg() {
    let result = -Vec3::new(1.0, -2.0, 3.0);
    assert!(approx_eq(result.x, -1.0));
    assert!(approx_eq(result.y, 2.0));
    assert!(approx_eq(result.z, -3.0));
}

#[test]
fn test_dot() {
    let result = Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(4.0, 5.0, 6.0));
    assert!(approx_eq(result, 32.0));
}

#[test]
fn test_length() {
    let result = Vec3::new(1.0, 2.0, 2.0).length();
    assert!(approx_eq(result, 3.0));
}

#[test]
fn test_length_squared() {
    let result = Vec3::new(1.0, 2.0, 2.0).length_squared();
    assert!(approx_eq(result, 9.0));
}

#[test]
fn test_normalize() {
    let result = Vec3::new(1.0, 2.0, 2.0).normalize();
    assert!(approx_eq(result.length(), 1.0));
}
