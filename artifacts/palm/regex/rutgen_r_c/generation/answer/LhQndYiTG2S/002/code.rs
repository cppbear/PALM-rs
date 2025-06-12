// Answer 0

#[test]
fn test_find_literals_unanchored() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct DummyNFA {
        prefixes: LiteralSearcher,
        is_anchored_start: bool,
    }

    struct DummyExecReadOnly {
        nfa: DummyNFA,
        suffixes: LiteralSearcher,
    }

    let literals = LiteralSearcher::empty();
    let nfa = DummyNFA {
        prefixes: literals.clone(),
        is_anchored_start: false,
    };
    let ro = DummyExecReadOnly {
        nfa,
        suffixes: literals,
    };

    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let text = b"hello world";
    let start = 0;

    assert_eq!(exec.find_literals(MatchLiteralType::Unanchored, text, start), Some((0, 0)));
}

#[test]
fn test_find_literals_anchored_start_false() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct DummyNFA {
        prefixes: LiteralSearcher,
        is_anchored_start: bool,
    }

    struct DummyExecReadOnly {
        nfa: DummyNFA,
        suffixes: LiteralSearcher,
    }

    let literals = LiteralSearcher::empty();
    let nfa = DummyNFA {
        prefixes: literals.clone(),
        is_anchored_start: true,
    };
    let ro = DummyExecReadOnly {
        nfa,
        suffixes: literals.clone(),
    };

    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let text = b"hello world";
    let start = 1; // start is non-zero

    assert_eq!(exec.find_literals(MatchLiteralType::AnchoredStart, text, start), None);
}

#[test]
fn test_find_literals_anchored_end() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct DummyNFA {
        prefixes: LiteralSearcher,
        is_anchored_start: bool,
    }

    struct DummyExecReadOnly {
        nfa: DummyNFA,
        suffixes: LiteralSearcher,
    }

    let suffixes = LiteralSearcher::empty();
    let nfa = DummyNFA {
        prefixes: LiteralSearcher::empty(),
        is_anchored_start: false,
    };
    let ro = DummyExecReadOnly {
        nfa,
        suffixes: suffixes.clone(),
    };

    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let text = b"world";
    let start = 0;

    assert_eq!(exec.find_literals(MatchLiteralType::AnchoredEnd, text, start), None);
}

