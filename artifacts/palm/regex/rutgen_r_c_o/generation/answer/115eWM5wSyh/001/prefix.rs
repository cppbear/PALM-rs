// Answer 0

#[test]
fn test_next_state_si_is_dead() {
    let mut qcur = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: Vec::new(),
        sparse: Vec::new(),
        size: 0,
    };
    let si = STATE_DEAD;
    let b = Byte(0);
    
    let prog = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    
    let cache = ProgramCache::new();
    
    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };
    
    let _ = fsm.next_state(&mut qcur, &mut qnext, si, b);
}

