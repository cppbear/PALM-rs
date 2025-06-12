// Answer 0

#[test]
fn test_find_literals_unanchored() {
    let text = b"Hello, world!";
    let searcher = LiteralSearcher::prefixes(Literals::from_strings(vec!["Hello".to_string()]));
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["Hello".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: searcher.clone(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    let result = exec.find_literals(MatchLiteralType::Unanchored, text, 0);
    assert_eq!(result, Some((0, 5))); // "Hello" match
}

#[test]
fn test_find_literals_anchored_start() {
    let text = b"Hello, world!";
    let searcher = LiteralSearcher::prefixes(Literals::from_strings(vec!["Hello".to_string()]));
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["Hello".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: searcher.clone(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    
    let result = exec.find_literals(MatchLiteralType::AnchoredStart, text, 0);
    assert_eq!(result, Some((0, 5))); // "Hello" match
    
    let result_not_found = exec.find_literals(MatchLiteralType::AnchoredStart, text, 1);
    assert_eq!(result_not_found, None);
}

#[test]
fn test_find_literals_anchored_end() {
    let text = b"Hello, world!";
    let searcher = LiteralSearcher::suffixes(Literals::from_strings(vec!["world!".to_string()]));
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["world!".to_string()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: searcher.clone(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    let result = exec.find_literals(MatchLiteralType::AnchoredEnd, text, 7);
    assert_eq!(result, Some((7, 13))); // "world!" match

    let result_not_found = exec.find_literals(MatchLiteralType::AnchoredEnd, text, 0);
    assert_eq!(result_not_found, None);
}

