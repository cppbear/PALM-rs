// Answer 0

#[derive(Debug, PartialEq)]
enum FlagsItemKind {
    Negation,
    Flag(Flag),
}

#[derive(Debug, PartialEq, Clone)]
struct Flag {
    name: String,
}

struct FlagsItem {
    kind: FlagsItemKind,
}

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
fn test_flag_state_with_positive_flag() {
    let flag = Flag { name: String::from("test_flag") };
    let flags = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(flag.clone()) },
        ],
    };
    assert_eq!(flags.flag_state(flag), Some(true));
}

#[test]
fn test_flag_state_with_negated_flag() {
    let flag = Flag { name: String::from("test_flag") };
    let flags = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
            FlagsItem { kind: FlagsItemKind::Flag(flag.clone()) },
        ],
    };
    assert_eq!(flags.flag_state(flag), Some(false));
}

#[test]
fn test_flag_state_with_nonexistent_flag() {
    let flag = Flag { name: String::from("nonexistent_flag") };
    let flags = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag { name: String::from("test_flag") }) },
        ],
    };
    assert_eq!(flags.flag_state(flag), None);
}

#[test]
fn test_flag_state_with_multiple_items() {
    let flag = Flag { name: String::from("test_flag") };
    let flags = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
            FlagsItem { kind: FlagsItemKind::Flag(flag.clone()) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag { name: String::from("another_flag") }) },
        ],
    };
    assert_eq!(flags.flag_state(flag), Some(false));
}

#[test]
fn test_flag_state_with_only_negation() {
    let flag = Flag { name: String::from("test_flag") };
    let flags = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
        ],
    };
    assert_eq!(flags.flag_state(flag), None);
}

