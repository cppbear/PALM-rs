// Answer 0

#[test]
fn test_unexpected_i8_min() {
    let content = Content::I8(-128);
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_i8_zero() {
    let content = Content::I8(0);
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_i8_max() {
    let content = Content::I8(127);
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_i8_negative() {
    let content = Content::I8(-50);
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_i8_positive() {
    let content = Content::I8(50);
    let _ = content.unexpected();
}

