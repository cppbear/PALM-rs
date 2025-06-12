// Answer 0

#[test]
fn test_many_matches_at_literal_match() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct MockExecReadOnly {
        nfa: Program,
        match_type: MatchType,
        suffixes: LiteralSearcher,
        is_anchored_end: bool,
    }

    struct MockExecNoSync {
        ro: Arc<MockExecReadOnly>,
        cache: ProgramCache,
    }

    let program = Program {
        insts: vec![], // Assuming empty for simplicity.
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let ro = Arc::new(MockExecReadOnly {
        nfa: program,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        suffixes: LiteralSearcher::default(),
        is_anchored_end: true,
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = MockExecNoSync { ro: ro.clone(), cache: cache };

    let mut matches = vec![false];
    let text = b"example text";
    
    assert!(exec.many_matches_at(&mut matches, text, 0));
    assert!(matches[0]);
}

#[test]
fn test_many_matches_at_no_match() {
    use std::sync::Arc;
    use std::cell::RefCell;

    let program = Program {
        insts: vec![], // Assuming empty for simplicity.
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    struct MockExecReadOnly {
        nfa: Program,
        match_type: MatchType,
        suffixes: LiteralSearcher,
        is_anchored_end: bool,
    }

    struct MockExecNoSync {
        ro: Arc<MockExecReadOnly>,
        cache: ProgramCache,
    }

    let ro = Arc::new(MockExecReadOnly {
        nfa: program,
        match_type: MatchType::Literal(MatchLiteralType::AnchoredStart),
        suffixes: LiteralSearcher::default(),
        is_anchored_end: true,
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = MockExecNoSync { ro, cache };

    let mut matches = vec![false];
    let text = b"wrong text";
    
    assert!(!exec.many_matches_at(&mut matches, text, 0));
    assert!(!matches[0]);
}

#[test]
fn test_many_matches_at_no_anchor_end() {
    use std::sync::Arc;
    use std::cell::RefCell;

    let program = Program {
        insts: vec![], // Assuming empty for simplification.
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    struct MockExecReadOnly {
        nfa: Program,
        match_type: MatchType,
        suffixes: LiteralSearcher,
        is_anchored_end: bool,
    }

    struct MockExecNoSync {
        ro: Arc<MockExecReadOnly>,
        cache: ProgramCache,
    }

    let ro = Arc::new(MockExecReadOnly {
        nfa: program,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        suffixes: LiteralSearcher::default(),
        is_anchored_end: false,
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = MockExecNoSync { ro, cache };

    let mut matches = vec![false];
    let text = b"any text";
    
    assert!(!exec.many_matches_at(&mut matches, text, 0));
    assert!(!matches[0]);
}

