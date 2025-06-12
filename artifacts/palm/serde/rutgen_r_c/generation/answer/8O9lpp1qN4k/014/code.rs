// Answer 0

#[test]
fn test_unexpected_signed_i64() {
    let content = Content::I64(42);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(42));
}

#[test]
fn test_unexpected_signed_i64_negative() {
    let content = Content::I64(-42);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(-42));
}

#[test]
fn test_unexpected_signed_i64_zero() {
    let content = Content::I64(0);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(0));
}

