// Answer 0

#[derive(Debug)]
enum ClassUnicodeOpKind {
    Equal,
    Colon,
    Other, // Some non-matching case
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
fn test_is_equal_false_for_other() {
    let op = ClassUnicodeOpKind::Other;
    assert_eq!(op.is_equal(), false);
}

#[test]
fn test_is_equal_false_for_empty() {
    let op = ClassUnicodeOpKind::Other; // Reusing to show another case
    assert_eq!(op.is_equal(), false);
}

