// Answer 0

#[test]
fn test_exec_at_reverse_quit() {
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
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let cache = ProgramCache::default();

    let mut sparse_cur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut sparse_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_MAX,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: vec![],
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };

    let text = [0];

    let result = fsm.exec_at_reverse(&mut sparse_cur, &mut sparse_next, &text);
}

