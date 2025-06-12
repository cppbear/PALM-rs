// Answer 0

#[derive(Clone, PartialEq)]
enum Flag {
    IgnoreCase,
    Multiline,
}

enum FlagsItemKind {
    Negation,
    Flag(Flag),
}

struct FlagsItem {
    kind: FlagsItemKind,
}

struct FlagsSet {
    items: Vec<FlagsItem>,
}

impl FlagsSet {
    pub fn new(items: Vec<FlagsItem>) -> Self {
        Self { items }
    }

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
fn test_flag_state_flag_not_set() {
    let flags_set = FlagsSet::new(vec![]);
    assert_eq!(flags_set.flag_state(Flag::IgnoreCase), None);
}

#[test]
fn test_flag_state_flag_set() {
    let flags_set = FlagsSet::new(vec![
        FlagsItem { kind: FlagsItemKind::Flag(Flag::IgnoreCase) },
    ]);
    assert_eq!(flags_set.flag_state(Flag::IgnoreCase), Some(true));
}

#[test]
fn test_flag_state_flag_negated() {
    let flags_set = FlagsSet::new(vec![
        FlagsItem { kind: FlagsItemKind::Negation },
        FlagsItem { kind: FlagsItemKind::Flag(Flag::IgnoreCase) },
    ]);
    assert_eq!(flags_set.flag_state(Flag::IgnoreCase), Some(false));
}

#[test]
fn test_flag_state_multiple_items() {
    let flags_set = FlagsSet::new(vec![
        FlagsItem { kind: FlagsItemKind::Negation },
        FlagsItem { kind: FlagsItemKind::Flag(Flag::IgnoreCase) },
        FlagsItem { kind: FlagsItemKind::Flag(Flag::Multiline) },
    ]);
    assert_eq!(flags_set.flag_state(Flag::IgnoreCase), Some(false));
    assert_eq!(flags_set.flag_state(Flag::Multiline), Some(true));
}

#[test]
fn test_flag_state_unrelated_items() {
    let flags_set = FlagsSet::new(vec![
        FlagsItem { kind: FlagsItemKind::Negation },
    ]);
    assert_eq!(flags_set.flag_state(Flag::Multiline), None);
}

