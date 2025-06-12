// Answer 0

#[test]
fn test_is_match_at_literal() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
    }

    let res = vec![String::from("a")];
    let captures = vec![None];
    let start = 0;
    
    let nfa_program = Program {
        insts: vec![],
        matches: vec![],
        captures: captures.clone(),
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
        dfa_size_limit: 0,
    };

    let dfa_program = nfa_program.clone();
    let dfa_reverse_program = nfa_program.clone();
    
    let ro = Arc::new(MockExecReadOnly {
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::default(),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    let matched = exec.is_match_at(b"a", start);
    assert!(matched);
}

#[test]
fn test_is_match_at_no_match() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
    }

    let res = vec![String::from("a")];
    let captures = vec![None];
    let start = 0;
    
    let nfa_program = Program {
        insts: vec![],
        matches: vec![],
        captures: captures.clone(),
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
        dfa_size_limit: 0,
    };

    let dfa_program = nfa_program.clone();
    let dfa_reverse_program = nfa_program.clone();
    
    let ro = Arc::new(MockExecReadOnly {
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::default(),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let matched = exec.is_match_at(b"b", start);
    assert!(!matched);
}

#[test]
fn test_is_match_at_with_anchored_start() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockExecReadOnly {
        match_type: MatchType,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
    }

    let res = vec![String::from("ab")];
    let captures = vec![None];
    let start = 0;
    
    let nfa_program = Program {
        insts: vec![],
        matches: vec![],
        captures: captures.clone(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let dfa_program = nfa_program.clone();
    let dfa_reverse_program = nfa_program.clone();
    
    let ro = Arc::new(MockExecReadOnly {
        match_type: MatchType::Literal(MatchLiteralType::AnchoredStart),
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_reverse_program,
        suffixes: LiteralSearcher::default(),
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let matched = exec.is_match_at(b"ab", start);
    assert!(matched);

    let no_match = exec.is_match_at(b"b", start);
    assert!(!no_match);
}

