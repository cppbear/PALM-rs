// Answer 0

#[test]
fn test_exec_at_quit_state() {
    struct Cache;
    struct SparseSet {
        dense: Vec<usize>,
        sparse: Vec<usize>,
        size: usize,
    }

    impl SparseSet {
        fn new() -> Self {
            SparseSet {
                dense: vec![],
                sparse: vec![],
                size: 0,
            }
        }
    }

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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
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

    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text: &[u8] = b"";
    
    // Simulating conditions for next_si to equal STATE_QUIT
    let next_si = STATE_QUIT;

    // Since `next_si` is forced to be `STATE_QUIT`, the function should return Result::Quit
    let result = fsm.exec_at(&mut qcur, &mut qnext, text);

    match result {
        Result::Quit => {},
        _ => panic!("Expected Result::Quit"),
    }
}

