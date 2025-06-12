// Answer 0

#[derive(Debug)]
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
fn test_is_negation_true() {
    let negation_item = FlagsItemKind::Negation;
    assert!(negation_item.is_negation());
}

#[test]
fn test_is_negation_false() {
    let other_item = FlagsItemKind::Other;
    assert!(!other_item.is_negation());
}

