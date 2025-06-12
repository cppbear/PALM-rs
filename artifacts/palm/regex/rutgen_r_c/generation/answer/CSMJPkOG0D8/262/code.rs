// Answer 0

#[test]
fn test_exec_at_with_all_constraints() {
    use std::collections::HashMap;

    let prog = Program {
        insts: vec![],
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
        dfa_size_limit: 10,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: vec![0; 10].into_boxed_slice() }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let text: &[u8] = b"test input";

    // This should satisfy the constraints outlined
    let result = fsm.exec_at(&mut qcur, &mut qnext, text);

    match result {
        Result::NoMatch(_) => panic!("Expected a match!"),
        Result::Match(pos) => assert!(pos < text.len()),
        Result::Quit => panic!("Unexpected early quit!"),
    }
}

#[test]
fn test_exec_at_with_no_match() {
    use std::collections::HashMap;

    let prog = Program {
        insts: vec![],
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
        dfa_size_limit: 10,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: vec![0; 10].into_boxed_slice() }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let text: &[u8] = b"abcd"; // Text that shouldn't match

    let result = fsm.exec_at(&mut qcur, &mut qnext, text);

    match result {
        Result::NoMatch(pos) => assert_eq!(pos, text.len()), // should return length of text
        Result::Match(_) => panic!("Expected no match!"),
        Result::Quit => panic!("Unexpected early quit!"),
    }
}

