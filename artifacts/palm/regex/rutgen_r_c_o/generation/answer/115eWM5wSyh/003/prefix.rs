// Answer 0

#[test]
fn test_next_state_with_si_not_dead_and_trans_next_returns_dead() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };
    
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {},
        dfa_size_limit: 1024,
    };

    let mut sparse_cur = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };

    let mut sparse_next = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let si = 2; // example state pointer
    let b = Byte(0); // example byte input

    fsm.cache.trans.set_next(si, fsm.byte_class(b), STATE_DEAD);

    let _result = fsm.next_state(&mut sparse_cur, &mut sparse_next, si, b);
}

#[test]
fn test_next_state_with_varied_si_and_trans_next_returns_dead() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(256),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };
    
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {},
        dfa_size_limit: 1024,
    };

    let mut sparse_cur = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };

    let mut sparse_next = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    for si in 2..=STATE_MAX {
        let b = Byte(1); // different byte input
        fsm.cache.trans.set_next(si, fsm.byte_class(b), STATE_DEAD);
        
        let _result = fsm.next_state(&mut sparse_cur, &mut sparse_next, si, b);
    }
}

