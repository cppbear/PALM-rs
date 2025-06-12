// Answer 0

#[test]
fn test_exec_at_with_match_and_quit() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
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
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let input: Vec<u8> = vec![1, 2, 3, 4]; // Test input for execution
    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();

    let result = unsafe { fsm.exec_at(&mut qcur, &mut qnext, &input) };
    assert!(matches!(result, Result::Match(_)));
}

#[test]
fn test_exec_at_with_next_si_exceeding_max() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
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
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text: Vec<u8> = vec![5, 6]; // Input text will maximize variables

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    // Setting next_si to STATE_MAX + 1 to exceed maximum
    let next_si = STATE_MAX + 1;
    if next_si > STATE_MAX {
        panic!("next_si exceeded maximum, expected behavior.");
    }

    let result = unsafe { fsm.exec_at(&mut qcur, &mut qnext, &text) };
    assert!(matches!(result, Result::Match(_)));
}

