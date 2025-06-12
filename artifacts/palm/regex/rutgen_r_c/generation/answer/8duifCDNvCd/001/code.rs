// Answer 0

#[test]
fn test_approximate_size_empty() {
    let prog = Program::new();
    let cache = CacheInner {
        compiled: HashMap::new(),
        trans: Vec::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };
    let fsm = Fsm {
        prog: &prog,
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
fn test_approximate_size_with_program() {
    let mut prog = Program::new();
    prog.insts.push(Inst::NoOp);
    prog.matches.push(0);
    prog.captures.push(Some("first".to_string()));
    prog.capture_name_idx.insert("first".to_string(), 0);
    let cache_size = 1024;
    let cache = CacheInner {
        compiled: HashMap::new(),
        trans: Vec::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: cache_size,
    };
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };
    assert_eq!(fsm.approximate_size(), cache_size + prog.approximate_size());
}

#[test]
fn test_approximate_size_large_cache() {
    let mut prog = Program::new();
    prog.insts = vec![Inst::NoOp; 100];
    prog.matches.push(1);
    let cache_size = 2048;
    let cache = CacheInner {
        compiled: HashMap::new(),
        trans: Vec::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 1,
        size: cache_size,
    };
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };
    assert_eq!(fsm.approximate_size(), cache_size + prog.approximate_size());
}

#[test]
fn test_approximate_size_with_varied_cache_sizes() {
    let mut prog = Program::new();
    prog.insts.push(Inst::NoOp);
    let cache_sizes = vec![0, 512, 1024, 2048];
    for &cache_size in &cache_sizes {
        let cache = CacheInner {
            compiled: HashMap::new(),
            trans: Vec::new(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: cache_size,
        };
        let fsm = Fsm {
            prog: &prog,
            start: STATE_START,
            at: 0,
            quit_after_match: false,
            last_match_si: STATE_UNKNOWN,
            last_cache_flush: 0,
            cache: &mut cache,
        };
        assert_eq!(fsm.approximate_size(), cache_size + prog.approximate_size());
    }
}

