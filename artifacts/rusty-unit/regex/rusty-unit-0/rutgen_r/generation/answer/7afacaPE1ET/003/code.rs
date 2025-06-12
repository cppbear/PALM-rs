// Answer 0

#[derive(Clone, PartialEq)]
enum Flag {
    FlagA,
    FlagB,
}

struct FlagsItem {
    kind: FlagsItemKind,
}

enum FlagsItemKind {
    Negation,
    Flag(Flag),
}

struct FlagsSet {
    items: Vec<FlagsItem>,
}

impl FlagsSet {
    pub fn flag_state(&self, flag: Flag) -> Option<bool> {
        let mut negated = false;
        for x in &self.items {
            match x.kind {
                FlagsItemKind::Negation => {
                    negated = true;
                }
                FlagsItemKind::Flag(ref xflag) if xflag == &flag => {
                    return Some(!negated);
                }
                _ => {}
            }
        }
        None
    }
}

#[test]
fn test_flag_state_with_negation() {
    let flags_set = FlagsSet {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::FlagB) },
        ],
    };
    assert_eq!(flags_set.flag_state(Flag::FlagA), None);
}

#[test]
fn test_flag_state_without_negation() {
    let flags_set = FlagsSet {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::FlagB) },
        ],
    };
    assert_eq!(flags_set.flag_state(Flag::FlagA), None);
}

#[test]
fn test_flag_state_with_multiple_items() {
    let flags_set = FlagsSet {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::FlagB) },
            FlagsItem { kind: FlagsItemKind::Negation },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::FlagB) },
        ],
    };
    assert_eq!(flags_set.flag_state(Flag::FlagA), None);
}

#[test]
fn test_flag_state_with_only_negation() {
    let flags_set = FlagsSet {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
        ],
    };
    assert_eq!(flags_set.flag_state(Flag::FlagA), None);
}

