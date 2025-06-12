// Answer 0

#[derive(Debug)]
enum ClassUnicodeOpKind {
    Equal,
    Colon,
    Other,
}

impl ClassUnicodeOpKind {
    pub fn is_equal(&self) -> bool {
        match *self {
            ClassUnicodeOpKind::Equal | ClassUnicodeOpKind::Colon => true,
            _ => false,
        }
    }
}

#[test]
fn test_class_unicode_op_kind_equal() {
    let op = ClassUnicodeOpKind::Equal;
    assert!(op.is_equal());
}

#[test]
fn test_class_unicode_op_kind_colon() {
    let op = ClassUnicodeOpKind::Colon;
    assert!(op.is_equal());
}

#[test]
fn test_class_unicode_op_kind_other() {
    let op = ClassUnicodeOpKind::Other;
    assert!(!op.is_equal());
}

