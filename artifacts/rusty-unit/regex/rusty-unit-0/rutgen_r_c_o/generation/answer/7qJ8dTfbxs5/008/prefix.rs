// Answer 0

#[test]
fn test_follow_epsilons_case1() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State::default(); 1],
        start_states: vec![STATE_START],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };

    let insts = vec![
        Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundaryAscii }),
        Inst::Match(0),
    ];

    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
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

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        word_boundary: true,
        ..Default::default()
    };

    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case2() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State::default(); 1],
        start_states: vec![STATE_START],
        stack: vec![2],
        flush_count: 0,
        size: 0,
    };

    let insts = vec![
        Inst::EmptyLook(InstEmptyLook { goto: 3, look: EmptyLook::WordBoundaryAscii }),
        Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundary }),
        Inst::Match(0),
    ];

    let program = Program {
        insts,
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

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        word_boundary: true,
        ..Default::default()
    };

    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case3() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State::default(); 1],
        start_states: vec![STATE_START],
        stack: vec![1],
        flush_count: 0,
        size: 0,
    };

    let insts = vec![
        Inst::Match(1),
        Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::NotWordBoundaryAscii }),
    ];

    let program = Program {
        insts,
        matches: vec![1],
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

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        word_boundary: false,
        ..Default::default()
    };

    fsm.follow_epsilons(1, &mut q, flags);
}

