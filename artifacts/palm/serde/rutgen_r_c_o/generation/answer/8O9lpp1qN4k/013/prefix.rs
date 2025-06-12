// Answer 0

#[test]
fn test_unexpected_float_zero() {
    let content = Content::F32(0.0);
    content.unexpected();
}

#[test]
fn test_unexpected_float_small_positive() {
    let content = Content::F32(1.5);
    content.unexpected();
}

#[test]
fn test_unexpected_float_large_positive() {
    let content = Content::F32(3.0);
    content.unexpected();
}

#[test]
fn test_unexpected_float_large() {
    let content = Content::F32(3.4028235e38);
    content.unexpected();
}

#[test]
fn test_unexpected_float_negative() {
    let content = Content::F32(-1.5);
    content.unexpected();
}

