// Answer 0

#[test]
fn test_continue_past_first_match_is_reverse_true() {
    struct MockProgram {
        is_reverse: bool,
        matches: Vec<u32>,
    }
    
    let mock_program = MockProgram {
        is_reverse: true,
        matches: vec![1],
    };

    let fsm = Fsm {
        prog: &mock_program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.continue_past_first_match(), true);
}

#[test]
fn test_continue_past_first_match_matches_len_greater_than_one() {
    struct MockProgram {
        is_reverse: bool,
        matches: Vec<u32>,
    }
    
    let mock_program = MockProgram {
        is_reverse: false,
        matches: vec![1, 2],
    };

    let fsm = Fsm {
        prog: &mock_program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.continue_past_first_match(), true);
}

#[test]
fn test_continue_past_first_match_is_reverse_false_and_one_match() {
    struct MockProgram {
        is_reverse: bool,
        matches: Vec<u32>,
    }
    
    let mock_program = MockProgram {
        is_reverse: false,
        matches: vec![1],
    };

    let fsm = Fsm {
        prog: &mock_program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.continue_past_first_match(), false);
}

