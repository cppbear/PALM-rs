// Answer 0

#[test]
fn test_exec_at_match() {
    use std::collections::HashMap;
    
    let program = Program {
        insts: vec![],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::from([0u8; 10]) }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text = b"example text input";
    let mut qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0; 256],
        size: 1,
    };
    let mut qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0; 256],
        size: 1,
    };

    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
    match result {
        Result::Match(at) => assert_eq!(at, text.len() - 1),
        _ => panic!("Expected a match result"),
    }
}

#[test]
fn test_exec_at_no_match() {
    use std::collections::HashMap;
    
    let program = Program {
        insts: vec![],
        matches: vec![0, 2],
        captures: vec![],
        capture_name_idx: std::sync::Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::from([0u8; 10]) }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text = b"no match input";
    let mut qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0; 256],
        size: 1,
    };
    let mut qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0; 256],
        size: 1,
    };

    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
    match result {
        Result::NoMatch(at) => assert_eq!(at, text.len()),
        _ => panic!("Expected a no match result"),
    }
}

