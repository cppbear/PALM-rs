// Answer 0

#[derive(Debug)]
struct FlagsItem {
    kind: FlagsItemKind,
}

#[derive(Debug, PartialEq)]
enum FlagsItemKind {
    Negation,
    Flag(Flag),
}

#[derive(Debug, PartialEq)]
struct Flag;

struct Flags {
    items: Vec<FlagsItem>,
}

impl Flags {
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
fn test_flag_state_with_negation_only() {
    let negation_item = FlagsItem {
        kind: FlagsItemKind::Negation,
    };
    let flags = Flags {
        items: vec![negation_item],
    };
    let flag = Flag;

    assert_eq!(flags.flag_state(flag), None);
}

#[test]
fn test_flag_state_with_no_matching_flag() {
    let negation_item = FlagsItem {
        kind: FlagsItemKind::Negation,
    };
    let non_matching_item = FlagsItem {
        kind: FlagsItemKind::Flag(Flag),
    };
    let flags = Flags {
        items: vec![negation_item, non_matching_item],
    };
    let flag = Flag;

    assert_eq!(flags.flag_state(flag), None);
}

#[test]
fn test_flag_state_with_match_and_negation() {
    let flag = Flag;
    let negation_item = FlagsItem {
        kind: FlagsItemKind::Negation,
    };
    let matching_item = FlagsItem {
        kind: FlagsItemKind::Flag(flag),
    };
    let flags = Flags {
        items: vec![negation_item, matching_item],
    };

    assert_eq!(flags.flag_state(flag), Some(false));
}

#[test]
fn test_flag_state_with_only_flag() {
    let flag = Flag;
    let matching_item = FlagsItem {
        kind: FlagsItemKind::Flag(flag),
    };
    let flags = Flags {
        items: vec![matching_item],
    };

    assert_eq!(flags.flag_state(flag), Some(true));
}

