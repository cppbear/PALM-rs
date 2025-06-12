// Answer 0

#[derive(Debug)]
enum ClassUnicodeOpKind {
    Equal,
    Colon,
    NotEqual,
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

#[test]
fn test_is_equal_with_other() {
    let op = ClassUnicodeOpKind::Other;
    assert!(!op.is_equal());
}

