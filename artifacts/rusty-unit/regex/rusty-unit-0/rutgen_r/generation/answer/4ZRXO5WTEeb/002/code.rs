// Answer 0

#[test]
fn test_is_match_with_match_instruction() {
    struct Inst {
        kind: InstKind,
    }

    enum InstKind {
        Match(bool),
        Other,
    }

    impl Inst {
        pub fn is_match(&self) -> bool {
            match *self {
                Inst { kind: InstKind::Match(_) } => true,
                _ => false,
            }
        }
    }

    let match_inst = Inst { kind: InstKind::Match(true) };
    assert!(match_inst.is_match());
}

#[test]
fn test_is_match_with_non_match_instruction() {
    struct Inst {
        kind: InstKind,
    }

    enum InstKind {
        Match(bool),
        Other,
    }

    impl Inst {
        pub fn is_match(&self) -> bool {
            match *self {
                Inst { kind: InstKind::Match(_) } => true,
                _ => false,
            }
        }
    }

    let other_inst = Inst { kind: InstKind::Other };
    assert!(!other_inst.is_match());
}

