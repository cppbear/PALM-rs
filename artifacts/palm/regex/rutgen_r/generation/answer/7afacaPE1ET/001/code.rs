// Answer 0

#[derive(Debug, PartialEq)]
enum FlagsItemKind {
    Negation,
    Flag(Flag),
}

#[derive(Debug, PartialEq)]
struct Flag;

struct FlagsItem {
    kind: FlagsItemKind,
}

struct FlagSet {
    items: Vec<FlagsItem>,
}

impl FlagSet {
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
fn test_flag_state_none_empty_set() {
    let flag_set = FlagSet { items: vec![] };
    let flag = Flag;
    assert_eq!(flag_set.flag_state(flag), None);
}

#[test]
fn test_flag_state_none_with_negation() {
    let flag_set = FlagSet { 
        items: vec![
            FlagsItem { kind: FlagsItemKind::Negation },
        ] 
    };
    let flag = Flag;
    assert_eq!(flag_set.flag_state(flag), None);
}

#[test]
fn test_flag_state_none_with_other_flags() {
    let flag_set = FlagSet { 
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag) }, // different flag
        ] 
    };
    let flag = Flag;
    assert_eq!(flag_set.flag_state(flag), None);
}

