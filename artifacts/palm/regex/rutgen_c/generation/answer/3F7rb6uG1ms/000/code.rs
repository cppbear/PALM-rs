// Answer 0

#[test]
fn test_is_equal_with_equal() {
    let op = ClassUnicodeOpKind::Equal;
    assert!(op.is_equal());
}

#[test]
fn test_is_equal_with_colon() {
    let op = ClassUnicodeOpKind::Colon;
    assert!(op.is_equal());
}

#[test]
fn test_is_equal_with_not_equal() {
    let op = ClassUnicodeOpKind::NotEqual;
    assert!(!op.is_equal());
}

