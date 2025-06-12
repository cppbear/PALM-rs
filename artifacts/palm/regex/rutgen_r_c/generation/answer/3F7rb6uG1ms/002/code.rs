// Answer 0

#[test]
fn test_is_equal_with_equal_variant() {
    let op = ClassUnicodeOpKind::Equal;
    assert!(op.is_equal());
}

#[test]
fn test_is_equal_with_colon_variant() {
    let op = ClassUnicodeOpKind::Colon;
    assert!(op.is_equal());
}

#[test]
fn test_is_equal_with_not_equal_variant() {
    let op = ClassUnicodeOpKind::NotEqual;
    assert!(!op.is_equal());
}

