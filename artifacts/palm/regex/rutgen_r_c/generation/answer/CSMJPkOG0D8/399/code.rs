// Answer 0

#[test]
fn test_exec_at_quit_condition() {
    use std::sync::Arc;

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([]) }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
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

    let mut qcur = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let mut qnext = SparseSet { dense: vec![], sparse: vec![], size: 0 };

    let text: &[u8] = b"abcde";

    // Set next_si to STATE_MAX and ensure the conditions for triggering Quit are met
    fsm.prog.matches = vec![0]; // only one match
    fsm.start = STATE_MAX; // prev_si == STATE_MAX
    let result = fsm.exec_at(&mut qcur, &mut qnext, text);

    if let Result::Quit = result {
        // Test passed
    } else {
        panic!("Test failed, expected Result::Quit but got: {:?}", result);
    }
}

