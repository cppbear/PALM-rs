// Answer 0

#[test]
fn test_approximate_size_empty_program() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let program = Program::new();
    
    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    assert_eq!(fsm.approximate_size(), 0);
}

#[test]
fn test_approximate_size_non_empty_program() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 128, // Simulating some cache size
    };

    let mut program = Program::new();
    program.insts.push(Inst::new()); // Simulating that we have added instructions
    program.matches.push(0);
    program.captures.push(None);
    
    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    assert_eq!(fsm.approximate_size(), 128 + program.approximate_size());
}

