use veld::{Vec2, math::Mat2};

#[test]
fn test_zero() {
    let mat = Mat2::ZERO;
    let res = Mat2::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    assert_eq!(mat, res);
}

#[test]
fn test_identity() {
    let mat = Mat2::IDENTITY;
    let res = Mat2::new(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0));

    assert_eq!(mat, res);
}

#[test]
fn test_new() {
    let mat = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -4.0));

    assert_eq!(mat.x_axis.x, 1.0);
    assert_eq!(mat.x_axis.y, 2.0);
    assert_eq!(mat.y_axis.x, -3.0);
    assert_eq!(mat.y_axis.y, -4.0);
}

#[test]
fn test_transpose() {
    let mat = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -4.0)).transpose();
    let res = Mat2::new(Vec2::new(1.0, -3.0), Vec2::new(2.0, -4.0));

    assert_eq!(mat, res);
}

#[test]
fn test_determinant() {
    let mat = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -4.0)).determinant();
    let res = 2.0;

    assert_eq!(mat, res);
}

#[test]
fn test_inverse() {
    let mat = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -4.0)).inverse();
    let res = Some(Mat2::new(Vec2::new(-2.0, -1.0), Vec2::new(1.5, 0.5)));

    assert_eq!(mat, res);
}

#[test]
fn test_add() {
    let a = Mat2::new(Vec2::new(5.5, 2.2), Vec2::new(-3.5, -4.4));
    let b = Mat2::new(Vec2::new(2.5, 3.8), Vec2::new(3.5, -2.4));
    let res = Mat2::new(Vec2::new(8.0, 6.0), Vec2::new(0.0, -6.8));

    assert_eq!(a + b, res);
}

#[test]
fn test_sub() {
    let a = Mat2::new(Vec2::new(5.5, 2.2), Vec2::new(-3.5, -4.4));
    let b = Mat2::new(Vec2::new(2.5, 3.2), Vec2::new(3.5, -2.4));
    let res = Mat2::new(Vec2::new(3.0, -1.0), Vec2::new(-7.0, -2.0));

    assert_eq!(a - b, res);
}

#[test]
fn test_mul() {
    let a = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -4.0));
    let b = Mat2::new(Vec2::new(-2.0, -1.0), Vec2::new(1.5, 0.5));
    let res = Mat2::IDENTITY;

    assert_eq!(a * b, res);
}

#[test]
fn test_mul_vec2() {
    let mat = Mat2::new(Vec2::new(1.5, -2.5), Vec2::new(-3.0, -4.0)) * Vec2::new(2.0, 1.0);
    let res = Vec2::new(0.0, -9.0);

    assert_eq!(mat, res);
}

#[test]
fn test_mul_f32() {
    let mat = Mat2::new(Vec2::new(1.5, -2.5), Vec2::new(-3.0, -4.0)) * 2.0;
    let res = Mat2::new(Vec2::new(3.0, -5.0), Vec2::new(-6.0, -8.0));

    assert_eq!(mat, res);
}

#[test]
fn test_div_f32() {
    let mat = Mat2::new(Vec2::new(1.0, -2.0), Vec2::new(-3.0, -4.0)) / 2.0;
    let res = Mat2::new(Vec2::new(0.5, -1.0), Vec2::new(-1.5, -2.0));

    assert_eq!(mat, res);
}

#[test]
fn test_neg() {
    let mat = -Mat2::new(Vec2::new(1.5, -2.5), Vec2::new(-3.0, -4.0));
    let res = Mat2::new(Vec2::new(-1.5, 2.5), Vec2::new(3.0, 4.0));

    assert_eq!(mat, res);
}
