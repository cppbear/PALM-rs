// Answer 0

#[test]
fn test_flag_state_none() {
    struct FlagsItem {
        kind: FlagsItemKind,
    }

    enum FlagsItemKind {
        Negation,
        Flag(Flag),
    }

    enum Flag {
        A,
        B,
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

    let flag_set = FlagSet { items: vec![] };
    let result = flag_set.flag_state(Flag::A);
    assert_eq!(result, None);
}

