// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct TestExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        suffixes: LiteralSearcher,
    }

    let read_only = Arc::new(TestExecReadOnly {
        match_type: MatchType::Nothing,
        nfa: Program::default(),
        suffixes: LiteralSearcher::default(),
    });
    
    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let mut locs = Locations(vec![None, None]);
    let text = b"some text";
    let start = 0;

    let result = exec.read_captures_at(&mut locs, text, start);
    assert_eq!(result, None);
}

#[test]
fn test_read_captures_at_two_slots() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct TestExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        suffixes: LiteralSearcher,
    }

    let read_only = Arc::new(TestExecReadOnly {
        match_type: MatchType::Dfa,
        nfa: Program::default(),
        suffixes: LiteralSearcher::default(),
    });

    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let mut locs = Locations(vec![None, None]);
    let text = b"some text";
    let start = 0;

    let result = exec.read_captures_at(&mut locs, text, start);
    assert!(result.is_some());
}

#[test]
fn test_read_captures_at_anchor_end_match() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct TestExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        suffixes: LiteralSearcher,
    }

    let read_only = Arc::new(TestExecReadOnly {
        match_type: MatchType::DfaAnchoredReverse,
        nfa: Program::default(),
        suffixes: LiteralSearcher::default(),
    });

    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let mut locs = Locations(vec![None, None]);
    let text = b"matching pattern";
    let start = 0;

    let result = exec.read_captures_at(&mut locs, text, start);
    assert!(result.is_some());
}

#[test]
fn test_read_captures_at_dfa_anchored_reverse_quit() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct TestExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        suffixes: LiteralSearcher,
    }

    let read_only = Arc::new(TestExecReadOnly {
        match_type: MatchType::DfaAnchoredReverse,
        nfa: Program::default(),
        suffixes: LiteralSearcher::default(),
    });

    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let mut locs = Locations(vec![None, None]);
    let text = b"test text";
    let start = 0;

    let result = exec.read_captures_at(&mut locs, text, start);
    assert!(result.is_none());
}

