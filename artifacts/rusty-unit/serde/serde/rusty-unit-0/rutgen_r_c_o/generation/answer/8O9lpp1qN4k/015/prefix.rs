// Answer 0

#[test]
fn test_unexpected_signed_i32_negative() {
    let content = Content::I32(-1);
    content.unexpected();
}

#[test]
fn test_unexpected_signed_i32_zero() {
    let content = Content::I32(0);
    content.unexpected();
}

#[test]
fn test_unexpected_signed_i32_positive() {
    let content = Content::I32(1);
    content.unexpected();
}

#[test]
fn test_unexpected_signed_i32_min() {
    let content = Content::I32(i32::MIN);
    content.unexpected();
}

#[test]
fn test_unexpected_signed_i32_max() {
    let content = Content::I32(i32::MAX);
    content.unexpected();
}

