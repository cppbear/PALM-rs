// Answer 0

#[test]
fn test_unexpected_float_min() {
    let content = Content::F64(::core::f64::MIN);
    content.unexpected();
}

#[test]
fn test_unexpected_float_max() {
    let content = Content::F64(::core::f64::MAX);
    content.unexpected();
}

#[test]
fn test_unexpected_float_zero() {
    let content = Content::F64(0.0);
    content.unexpected();
}

#[test]
fn test_unexpected_float_negative() {
    let content = Content::F64(-1.23);
    content.unexpected();
}

#[test]
fn test_unexpected_float_pi() {
    let content = Content::F64(3.141592653589793);
    content.unexpected();
}

#[test]
fn test_unexpected_float_e() {
    let content = Content::F64(2.718281828459045);
    content.unexpected();
}

