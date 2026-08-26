use veld::Mat4;
use veld::Vec4;

const EPSILON: f32 = 1e-6;
fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn test_new() {
    let m = Mat4::new(
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    assert!(approx_eq(m.x_axis, Vec4::new(1.0, 2.0, 3.0, 4.0)));
    assert!(approx_eq(m.y_axis, Vec4::new(5.0, 6.0, 7.0, 8.0)));
    assert!(approx_eq(m.z_axis, Vec4::new(9.0, 10.0, 11.0, 12.0)));
    assert!(approx_eq(m.w_axis, Vec4::new(13.0, 14.0, 15.0, 16.0)));
}

#[test]
fn test_identity() {
    let m = Mat4::identity();
    assert!(approx_eq(m.x_axis, Vec4::new(1.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(m.y_axis, Vec4::new(0.0, 1.0, 0.0, 0.0)));
    assert!(approx_eq(m.z_axis, Vec4::new(0.0, 0.0, 1.0, 0.0)));
    assert!(approx_eq(m.w_axis, Vec4::new(0.0, 0.0, 0.0, 1.0)));
}

#[test]
fn test_add() {
    let a = Mat4::identity();
    let b = Mat4::identity();
    let result = a + b;
    assert!(approx_eq(result.x_axis, Vec4::new(2.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.y_axis, Vec4::new(0.0, 2.0, 0.0, 0.0)));
    assert!(approx_eq(result.z_axis, Vec4::new(0.0, 0.0, 2.0, 0.0)));
    assert!(approx_eq(result.w_axis, Vec4::new(0.0, 0.0, 0.0, 2.0)));
}

#[test]
fn test_sub() {
    let a = Mat4::identity();
    let b = Mat4::identity();
    let result = a - b;
    assert!(approx_eq(result.x_axis, Vec4::new(0.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.y_axis, Vec4::new(0.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.z_axis, Vec4::new(0.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.w_axis, Vec4::new(0.0, 0.0, 0.0, 0.0)));
}

#[test]
fn test_mul_mat4() {
    let a = Mat4::identity();
    let b = Mat4::identity();
    let result = a * b;
    assert!(approx_eq(result, Mat4::identity()));
}

#[test]
fn test_mul_scalar() {
    let m = Mat4::identity();
    let result = m * 2.0;
    assert!(approx_eq(result.x_axis, Vec4::new(2.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.y_axis, Vec4::new(0.0, 2.0, 0.0, 0.0)));
    assert!(approx_eq(result.z_axis, Vec4::new(0.0, 0.0, 2.0, 0.0)));
    assert!(approx_eq(result.w_axis, Vec4::new(0.0, 0.0, 0.0, 2.0)));
}

#[test]
fn test_div_scalar() {
    let m = Mat4::new(
        Vec4::new(2.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 4.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 6.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 8.0),
    );
    let result = m / 2.0;
    assert!(approx_eq(result.x_axis, Vec4::new(1.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.y_axis, Vec4::new(0.0, 2.0, 0.0, 0.0)));
    assert!(approx_eq(result.z_axis, Vec4::new(0.0, 0.0, 3.0, 0.0)));
    assert!(approx_eq(result.w_axis, Vec4::new(0.0, 0.0, 0.0, 4.0)));
}

#[test]
fn test_neg() {
    let m = Mat4::identity();
    let result = -m;
    assert!(approx_eq(result.x_axis, Vec4::new(-1.0, 0.0, 0.0, 0.0)));
    assert!(approx_eq(result.y_axis, Vec4::new(0.0, -1.0, 0.0, 0.0)));
    assert!(approx_eq(result.z_axis, Vec4::new(0.0, 0.0, -1.0, 0.0)));
    assert!(approx_eq(result.w_axis, Vec4::new(0.0, 0.0, 0.0, -1.0)));
}
