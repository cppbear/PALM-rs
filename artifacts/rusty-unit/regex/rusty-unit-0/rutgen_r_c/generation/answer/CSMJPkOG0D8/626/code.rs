// Answer 0

#[test]
fn test_exec_at_no_match() {
    let program = Program {
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
        prefixes: Default::default(),
        dfa_size_limit: 0,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Default::default(),
        states: vec![],
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

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let result = unsafe {
        fsm.exec_at(&mut qcur, &mut qnext, &[])
    };

    match result {
        Result::NoMatch(_) => (),
        _ => panic!("Expected NoMatch"),
    }
}

#[test]
fn test_exec_at_match_found() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
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
        prefixes: Default::default(),
        dfa_size_limit: 0,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Default::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let result = unsafe {
        fsm.exec_at(&mut qcur, &mut qnext, &[0])
    };

    match result {
        Result::Match(_) => (),
        _ => panic!("Expected Match"),
    }
}

#[test]
fn test_exec_at_dead_state() {
    let program = Program {
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
        prefixes: Default::default(),
        dfa_size_limit: 0,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Default::default(),
        states: vec![],
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

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let result = unsafe {
        fsm.exec_at(&mut qcur, &mut qnext, &[1])
    };

    match result {
        Result::NoMatch(_) => (),
        _ => panic!("Expected NoMatch when in STATE_DEAD"),
    }
}

#[test]
fn test_exec_at_eof_transition() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
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
        prefixes: Default::default(),
        dfa_size_limit: 0,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Default::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let result = unsafe {
        fsm.exec_at(&mut qcur, &mut qnext, &[0])
    };

    match result {
        Result::Match(len) if len == 1 => (),
        _ => panic!("Expected Match with length 1"),
    }
}

