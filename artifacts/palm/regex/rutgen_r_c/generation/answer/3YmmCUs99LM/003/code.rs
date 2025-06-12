// Answer 0

#[test]
fn test_forward_no_match() {
    // Create a dummy Program with no matches
    let prog = Program {
        insts: vec![],
        matches: vec![],
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
        dfa_size_limit: 0,
    };

    // Create a dummy ProgramCache with a Simple sparse set
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN], // make start state unknown
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCache {
        inner: cache_inner,
        qcur: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
        qnext: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
    };

    // Define the input text and the starting position
    let text = b"abcdef";
    let at = 0;
    let quit_after_match = false;

    // Call the forward function
    let result = Fsm::forward(&prog, &cache, quit_after_match, text, at);

    // Check the result is NoMatch with the given at position
    assert_eq!(result, Result::NoMatch(at));
}

#[test]
fn test_forward_dead_state() {
    // Create a dummy Program with a match state but simulate a dead state condition
    let prog = Program {
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
        dfa_size_limit: 0,
    };

    // Create a dummy ProgramCache with a state that is dead
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_DEAD], // simulate a start state as dead
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCache {
        inner: cache_inner,
        qcur: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
        qnext: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
    };

    // Define the input text and the starting position
    let text = b"abcdef";
    let at = 0;
    let quit_after_match = false;

    // Call the forward function
    let result = Fsm::forward(&prog, &cache, quit_after_match, text, at);

    // Check the result is NoMatch with the given at position
    assert_eq!(result, Result::NoMatch(at));
}

