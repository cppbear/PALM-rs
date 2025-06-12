// Answer 0

#[test]
fn test_is_equal_equal() {
    let op = ClassUnicodeOpKind::Equal;
    op.is_equal();
}

#[test]
fn test_is_equal_colon() {
    let op = ClassUnicodeOpKind::Colon;
    op.is_equal();
}

#[test]
fn test_is_equal_not_equal() {
    let op = ClassUnicodeOpKind::NotEqual;
    op.is_equal();
}

