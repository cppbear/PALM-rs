// Answer 0

#[test]
fn test_follow_epsilons_valid_conditions() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook {
                goto: 1,
                look: EmptyLook::StartLine,
            }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: Vec::new(),
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut q = SparseSet::new(100);
    let flags = EmptyFlags {
        start: true,
        start_line: true,
        ..Default::default()
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
fn test_follow_epsilons_with_multiple_empty_looks() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook {
                goto: 1,
                look: EmptyLook::StartLine,
            }),
            Inst::EmptyLook(InstEmptyLook {
                goto: 2,
                look: EmptyLook::EndLine,
            }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: Vec::new(),
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut q = SparseSet::new(100);
    let flags = EmptyFlags {
        start_line: true,
        ..Default::default()
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
#[should_panic]
fn test_follow_epsilons_invalid_ip() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook {
                goto: 1,
                look: EmptyLook::StartLine,
            }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: Vec::new(),
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut q = SparseSet::new(100);
    let flags = EmptyFlags {
        start_line: true,
        ..Default::default()
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

    fsm.follow_epsilons(2, &mut q, flags); // This should panic as ip is invalid
}

