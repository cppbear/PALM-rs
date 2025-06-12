// Answer 0

#[test]
fn test_forward_many_single_match() {
    use std::collections::HashMap;
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([]) }],
        start_states: vec![0],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCache { inner: cache_inner };

    let text = b"abc";
    let mut matches = vec![false];

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
    
    assert!(result.is_match());
    assert_eq!(matches, vec![true]);
}

#[test]
fn test_forward_many_multiple_matches() {
    use std::collections::HashMap;
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([]) }, State { data: Box::new([]) }],
        start_states: vec![1],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCache { inner: cache_inner };

    let text = b"abcd";
    let mut matches = vec![false, false];

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
    
    assert!(result.is_match());
    assert_eq!(matches, vec![true, true]);
}

#[test]
fn test_forward_many_no_match() {
    use std::collections::HashMap;
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![1],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCache { inner: cache_inner };

    let text = b"xyz";
    let mut matches = vec![];

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
    
    assert!(!result.is_match());
}

#[test]
#[should_panic]
fn test_forward_many_panic_on_length_mismatch() {
    use std::collections::HashMap;
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([]) }],
        start_states: vec![0],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCache { inner: cache_inner };

    let text = b"abc";
    let mut matches = vec![false, false]; // Length mismatch

    let _ = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
}

