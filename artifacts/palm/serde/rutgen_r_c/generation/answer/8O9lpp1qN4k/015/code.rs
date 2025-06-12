// Answer 0

#[test]
fn test_unexpected_signed_i32() {
    let content = Content::I32(-42);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(-42));
}

#[test]
fn test_unexpected_signed_i32_zero() {
    let content = Content::I32(0);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(0));
}

#[test]
fn test_unexpected_signed_i32_max() {
    let content = Content::I32(i32::MAX);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(i32::MAX as i64));
}

#[test]
fn test_unexpected_signed_i32_min() {
    let content = Content::I32(i32::MIN);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(i32::MIN as i64));
}

