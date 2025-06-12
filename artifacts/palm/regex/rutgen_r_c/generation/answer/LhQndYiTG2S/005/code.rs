// Answer 0

#[test]
fn test_find_literals_unanchored() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    });
    
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"hello world";
    let result = exec.find_literals(MatchLiteralType::Unanchored, text, 0);
    assert_eq!(result, Some((0, 5)));
}

#[test]
fn test_find_literals_anchored_start_not_anchored() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: Program {
            prefixes: Literals::from(vec![b"hello".to_vec()]),
            is_anchored_start: false,
            ..Default::default()
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"hello world";
    let result = exec.find_literals(MatchLiteralType::AnchoredStart, text, 0);
    assert_eq!(result, Some((0, 5)));

    let result_offset = exec.find_literals(MatchLiteralType::AnchoredStart, text, 1);
    assert_eq!(result_offset, None);
}

#[test]
fn test_find_literals_anchored_end() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["world".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::suffixes(Literals::from(vec![b"world".to_vec()])),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"hello world";
    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, 6);
    assert_eq!(result, Some((6, 11)));

    let result_invalid_start = exec.find_literals(MatchLiteralType::AnchoredEnd, text, 0);
    assert_eq!(result_invalid_start, None);
}

#[test]
#[should_panic]
fn test_find_literals_invalid_start() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: Program {
            prefixes: Literals::from(vec![b"hello".to_vec()]),
            is_anchored_start: false,
            ..Default::default()
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"hello world";
    // This will panic as the range will be out of bounds
    let _result = exec.find_literals(MatchLiteralType::Unanchored, text, 15);
}

