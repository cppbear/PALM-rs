// Answer 0

#[test]
fn test_follow_epsilons_case1() {
    let mut dense = vec![0; 10];
    let mut sparse = vec![0; 256];
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![1], // Starting with a valid ip
        flush_count: 0,
        size: 0,
    };

    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundaryAscii }),
            Inst::Match(0),
        ],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 128,
    };

    let mut q = SparseSet::new(256);
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case2() {
    let mut dense = vec![0; 10];
    let mut sparse = vec![0; 256];
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![1], // Starting with a valid ip
        flush_count: 0,
        size: 0,
    };

    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: true,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartLine }),
            Inst::Match(0),
        ],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 128,
    };

    let mut q = SparseSet::new(256);
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case3() {
    let mut dense = vec![0; 10];
    let mut sparse = vec![0; 256];
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![1], // Starting with a valid ip
        flush_count: 0,
        size: 0,
    };

    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::NotWordBoundaryAscii }),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundaryAscii }),
            Inst::Match(0),
        ],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 128,
    };

    let mut q = SparseSet::new(256);
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(1, &mut q, flags);
}

