// Answer 0

#[test]
fn test_follow_epsilons_case1() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![2],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::NotWordBoundary }),
            Inst::Bytes(InstBytes::default()),
            Inst::Split(InstSplit { goto1: 1, goto2: 2 }),
        ],
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
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

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: true };
    
    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case2() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![3],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundary }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::NotWordBoundary }),
            Inst::Bytes(InstBytes::default()),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
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

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: true, not_word_boundary: true };

    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case3() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: vec![1],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::WordBoundary }),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::NotWordBoundary }),
            Inst::Match(1),
        ],
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
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

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    fsm.follow_epsilons(0, &mut q, flags);
}

