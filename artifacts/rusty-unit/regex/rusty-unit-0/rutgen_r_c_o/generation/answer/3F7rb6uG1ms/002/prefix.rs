// Answer 0

#[test]
fn test_class_unicode_op_kind_equal() {
    let op_kind = ClassUnicodeOpKind::Equal;
    op_kind.is_equal();
}

#[test]
fn test_class_unicode_op_kind_colon() {
    let op_kind = ClassUnicodeOpKind::Colon;
    op_kind.is_equal();
}

