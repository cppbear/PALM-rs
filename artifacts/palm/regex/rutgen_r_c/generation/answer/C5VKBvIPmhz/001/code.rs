// Answer 0

#[test]
fn test_next_after_empty_with_zero() {
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let result = exec.next_after_empty(b"test", 0);
    assert_eq!(result, 1);
}

#[test]
fn test_next_after_empty_with_positive_index() {
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let result = exec.next_after_empty(b"test", 5);
    assert_eq!(result, 6);
}

#[test]
fn test_next_after_empty_with_large_index() {
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let large_index = usize::MAX - 1; // to test upper boundary conditions
    let result = exec.next_after_empty(b"test", large_index);
    assert_eq!(result, large_index + 1);
}

