// Answer 0

#[derive(Debug)]
enum FlagsItemKind {
    Negation,
    Other,
}

struct FlagsItem {
    kind: FlagsItemKind,
}

impl FlagsItem {
    pub fn is_negation(&self) -> bool {
        match self.kind {
            FlagsItemKind::Negation => true,
            _ => false,
        }
    }
}

#[test]
fn test_is_negation_false_for_other() {
    let item = FlagsItem {
        kind: FlagsItemKind::Other,
    };
    assert_eq!(item.is_negation(), false);
}

#[test]
fn test_is_negation_false_for_another_non_negation() {
    let item = FlagsItem {
        kind: FlagsItemKind::Other,
    };
    assert_eq!(item.is_negation(), false);
}

