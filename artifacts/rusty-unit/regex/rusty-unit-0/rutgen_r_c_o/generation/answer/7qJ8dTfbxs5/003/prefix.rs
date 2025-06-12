// Answer 0

#[test]
fn test_follow_epsilons_case_1() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::new(); 10],
        start_states: vec![0; 10],
        stack: vec![1],
        flush_count: 0,
        size: 1,
    };

    let prog = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 2 }), Inst::Match(1), Inst::Bytes(InstBytes { goto: 3 })],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'a'],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: true,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case_2() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::new(); 10],
        start_states: vec![0; 10],
        stack: vec![0],
        flush_count: 0,
        size: 1,
    };

    let prog = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1 }), Inst::Bytes(InstBytes { goto: 2 }), Inst::Match(1)],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'b'],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    
    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start: true,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: true,
        not_word_boundary: false,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    fsm.follow_epsilons(1, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case_3() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::new(); 10],
        start_states: vec![0; 10],
        stack: vec![2],
        flush_count: 0,
        size: 1,
    };

    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Bytes(InstBytes { goto: 3 }), Inst::Bytes(InstBytes { goto: 4 })],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'c'],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start: true,
        end: true,
        start_line: false,
        end_line: true,
        word_boundary: false,
        not_word_boundary: true,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    fsm.follow_epsilons(2, &mut q, flags);
}

