// Answer 0

#[test]
fn test_exec_at_with_empty_text() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
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
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let text = b"";
    let _result = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_with_full_capacity() {
    let mut matches = vec![2];
    let prog = Program {
        insts: vec![Inst::new()],
        matches: matches.clone(),
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0, 1],
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
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: repeat(0).take(256).collect(),
        sparse: repeat(0).take(256).collect(),
        size: 256,
    };

    let mut qnext = SparseSet {
        dense: repeat(1).take(256).collect(),
        sparse: repeat(1).take(256).collect(),
        size: 256,
    };

    let text = b"abc";
    let _result = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_with_maximum_text_length() {
    let prog = Program {
        insts: vec![Inst::new(); 32],
        matches: vec![5],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
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
        at: 1024,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0; 256],
        sparse: vec![0; 256],
        size: 256,
    };

    let mut qnext = SparseSet {
        dense: vec![0; 256],
        sparse: vec![0; 256],
        size: 256,
    };

    let text = b"abcde" + &vec![0; 1018];
    let _result = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_with_dead_states() {
    let prog = Program {
        insts: vec![Inst::new()],
        matches: vec![1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
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
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let text = b"dead_state_input";
    let _result = fsm.exec_at(&mut qcur, &mut qnext, text);
}

