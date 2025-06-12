// Answer 0

#[test]
fn test_exec_at_reverse() {
    use std::collections::HashMap;

    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![], 
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 5,
        quit_after_match: true,
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

    let text = b"example";

    assert_eq!(fsm.exec_at_reverse(&mut qcur, &mut qnext, text), Result::Match(3));
}

#[test]
fn test_exec_at_reverse_no_match() {
    use std::collections::HashMap;

    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![], 
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 5,
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

    let text = b"no_match_case";

    assert_eq!(fsm.exec_at_reverse(&mut qcur, &mut qnext, text), Result::NoMatch(5));
}

