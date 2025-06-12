// Answer 0

#[test]
fn test_unexpected_with_i64_min() {
    let content = Content::I64(-9223372036854775808);
    content.unexpected();
}

#[test]
fn test_unexpected_with_i64_zero() {
    let content = Content::I64(0);
    content.unexpected();
}

#[test]
fn test_unexpected_with_i64_positive() {
    let content = Content::I64(123456789);
    content.unexpected();
}

#[test]
fn test_unexpected_with_i64_max() {
    let content = Content::I64(9223372036854775807);
    content.unexpected();
}

