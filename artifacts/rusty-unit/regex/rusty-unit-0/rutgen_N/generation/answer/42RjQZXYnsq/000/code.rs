// Answer 0

#[derive(Debug, PartialEq)]
enum FlagsItemKind {
    Negation,
    Other,
}

impl FlagsItemKind {
    pub fn is_negation(&self) -> bool {
        match *self {
            FlagsItemKind::Negation => true,
            _ => false,
        }
    }
}

#[test]
fn test_is_negation_with_negation() {
    let item = FlagsItemKind::Negation;
    assert!(item.is_negation());
}

#[test]
fn test_is_negation_with_other() {
    let item = FlagsItemKind::Other;
    assert!(!item.is_negation());
}

