// Answer 0

#[derive(Default)]
struct Prog {
    is_reverse: bool,
    prefixes: Vec<String>,
    is_anchored_start: bool,
}

struct Regex {
    prog: Prog,
}

impl Regex {
    fn has_prefix(&self) -> bool {
        !self.prog.is_reverse
            && !self.prog.prefixes.is_empty()
            && !self.prog.is_anchored_start
    }
}

#[test]
fn test_has_prefix_true() {
    let regex = Regex {
        prog: Prog {
            is_reverse: false,
            prefixes: vec!["abc".to_string()],
            is_anchored_start: false,
        },
    };
    assert_eq!(regex.has_prefix(), true);
}

#[test]
fn test_has_prefix_reverse() {
    let regex = Regex {
        prog: Prog {
            is_reverse: true,
            prefixes: vec!["abc".to_string()],
            is_anchored_start: false,
        },
    };
    assert_eq!(regex.has_prefix(), false);
}

#[test]
fn test_has_prefix_empty_prefixes() {
    let regex = Regex {
        prog: Prog {
            is_reverse: false,
            prefixes: vec![],
            is_anchored_start: false,
        },
    };
    assert_eq!(regex.has_prefix(), false);
}

#[test]
fn test_has_prefix_anchored_start() {
    let regex = Regex {
        prog: Prog {
            is_reverse: false,
            prefixes: vec!["abc".to_string()],
            is_anchored_start: true,
        },
    };
    assert_eq!(regex.has_prefix(), false);
}

