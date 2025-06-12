// Answer 0

#[test]
fn test_next_state_case_1() {
    let insts = Vec::new();
    let matches = Vec::new();
    let captures = Vec::new();
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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

    let mut qcur = SparseSet { dense: Vec::new(), sparse: Vec::new(), size: 0 };
    let mut qnext = SparseSet { dense: Vec::new(), sparse: Vec::new(), size: 0 };
    let si: StatePtr = 0;
    let b = Byte(100);

    fsm.next_state(&mut qcur, &mut qnext, si, b);
}

#[test]
fn test_next_state_case_2() {
    let insts = Vec::new();
    let matches = Vec::new();
    let captures = Vec::new();
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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

    let mut qcur = SparseSet { dense: Vec::new(), sparse: Vec::new(), size: 0 };
    let mut qnext = SparseSet { dense: Vec::new(), sparse: Vec::new(), size: 0 };
    let si: StatePtr = 1;
    let b = Byte(200);

    fsm.next_state(&mut qcur, &mut qnext, si, b);
}

#[test]
fn test_next_state_case_3() {
    let insts = Vec::new();
    let matches = Vec::new();
    let captures = Vec::new();
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
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

    let mut qcur = SparseSet { dense: Vec::new(), sparse: Vec::new(), size: 0 };
    let mut qnext = SparseSet { dense: Vec::new(), sparse: Vec::new(), size: 0 };
    let si: StatePtr = STATE_MAX;
    let b = Byte(150);

    fsm.next_state(&mut qcur, &mut qnext, si, b);
}

