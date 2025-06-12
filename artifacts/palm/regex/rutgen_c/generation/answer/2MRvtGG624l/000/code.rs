// Answer 0

#[test]
fn test_find_at_literal_match() {
    use std::cell::RefCell;
    use std::sync::Arc;
    
    struct DummyMatchType;
    impl MatchType {
        const Literal: DummyMatchType = DummyMatchType;
    }
    
    struct DummyProgramCache;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("abc")],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: DummyMatchType,
    });
    let cache = RefCell::new(DummyProgramCache);
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.find_at(b"abcde", 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_find_at_no_match() {
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DummyMatchType;
    impl MatchType {
        const Literal: DummyMatchType = DummyMatchType;
    }

    struct DummyProgramCache;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("xyz")],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: DummyMatchType,
    });
    let cache = RefCell::new(DummyProgramCache);
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.find_at(b"abcde", 0);
    assert_eq!(result, None);
}

#[test]
fn test_find_at_empty_text() {
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DummyMatchType;
    impl MatchType {
        const Literal: DummyMatchType = DummyMatchType;
    }

    struct DummyProgramCache;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("abc")],
        nfa: Program::new(),
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::new(),
        match_type: DummyMatchType,
    });
    let cache = RefCell::new(DummyProgramCache);
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    
    let result = exec.find_at(b"", 0);
    assert_eq!(result, None);
}

