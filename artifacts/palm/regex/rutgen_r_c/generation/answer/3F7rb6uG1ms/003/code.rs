// Answer 0

#[test]
fn test_is_equal_equal() {
    let op = ClassUnicodeOpKind::Equal;
    assert!(op.is_equal());
}

#[test]
fn test_is_equal_colon() {
    let op = ClassUnicodeOpKind::Colon;
    assert!(op.is_equal());
}

#[test]
fn test_is_equal_not_equal() {
    let op = ClassUnicodeOpKind::NotEqual;
    assert!(!op.is_equal());
}

