use veld::Vec4;

#[test]
fn test_new() {
    let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
    assert_eq!(v.w, 4.0);
}

#[test]
fn test_add() {
    let result = Vec4::new(1.0, 2.0, 3.0, 4.0) + Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert_eq!(result.x, 6.0);
    assert_eq!(result.y, 8.0);
    assert_eq!(result.z, 10.0);
    assert_eq!(result.w, 12.0);
}

#[test]
fn test_sub() {
    let result = Vec4::new(5.0, 6.0, 7.0, 8.0) - Vec4::new(2.0, 3.0, 4.0, 5.0);
    assert_eq!(result.x, 3.0);
    assert_eq!(result.y, 3.0);
    assert_eq!(result.z, 3.0);
    assert_eq!(result.w, 3.0);
}

#[test]
fn test_mul_vec4() {
    let result = Vec4::new(2.0, 3.0, 4.0, 5.0) * Vec4::new(6.0, 7.0, 8.0, 9.0);
    assert_eq!(result.x, 12.0);
    assert_eq!(result.y, 21.0);
    assert_eq!(result.z, 32.0);
    assert_eq!(result.w, 45.0);
}

#[test]
fn test_mul_scalar() {
    let result = Vec4::new(2.0, 3.0, 4.0, 5.0) * 2.0;
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 6.0);
    assert_eq!(result.z, 8.0);
    assert_eq!(result.w, 10.0);
}

#[test]
fn test_div_vec4() {
    let result = Vec4::new(8.0, 9.0, 10.0, 12.0) / Vec4::new(2.0, 3.0, 5.0, 4.0);
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 3.0);
    assert_eq!(result.z, 2.0);
    assert_eq!(result.w, 3.0);
}

#[test]
fn test_div_scalar() {
    let result = Vec4::new(6.0, 8.0, 10.0, 12.0) / 2.0;
    assert_eq!(result.x, 3.0);
    assert_eq!(result.y, 4.0);
    assert_eq!(result.z, 5.0);
    assert_eq!(result.w, 6.0);
}

#[test]
fn test_neg() {
    let result = -Vec4::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(result.x, -1.0);
    assert_eq!(result.y, 2.0);
    assert_eq!(result.z, -3.0);
    assert_eq!(result.w, 4.0);
}

#[test]
fn test_dot() {
    let result = Vec4::new(1.0, 2.0, 3.0, 4.0).dot(Vec4::new(5.0, 6.0, 7.0, 8.0));
    assert_eq!(result, 70.0);
}

#[test]
fn test_length() {
    let result = Vec4::new(1.0, 2.0, 2.0, 0.0).length();
    assert_eq!(result, 3.0);
}

#[test]
fn test_length_squared() {
    let result = Vec4::new(1.0, 2.0, 2.0, 0.0).length_squared();
    assert_eq!(result, 9.0);
}

#[test]
fn test_normalize() {
    let result = Vec4::new(1.0, 2.0, 2.0, 0.0).normalize();
    assert_eq!(result.length(), 1.0);
}
