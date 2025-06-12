// Answer 0

#[test]
fn test_follow_epsilons_case_1() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundary }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::WordBoundary }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let mut empty_flags = EmptyFlags { word_boundary: false, ..Default::default() };
    let mut sparse_set = SparseSet::new(10);
    let ip = 0;

    let mut fsm = Fsm { prog: &prog, start: 0, at: 0, quit_after_match: false, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    fsm.follow_epsilons(ip, &mut sparse_set, empty_flags);
}

#[test]
fn test_follow_epsilons_case_2() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 4, look: EmptyLook::WordBoundary }),
            Inst::EmptyLook(InstEmptyLook { goto: 5, look: EmptyLook::NotWordBoundary }),
            Inst::EmptyLook(InstEmptyLook { goto: 6, look: EmptyLook::WordBoundary }),
            Inst::Match(1),
        ],
        matches: vec![1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let mut empty_flags = EmptyFlags { word_boundary: false, ..Default::default() };
    let mut sparse_set = SparseSet::new(20);
    let ip = 0;

    let mut fsm = Fsm { prog: &prog, start: 0, at: 0, quit_after_match: false, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    fsm.follow_epsilons(ip, &mut sparse_set, empty_flags);
}

#[test]
fn test_follow_epsilons_case_3() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 8, look: EmptyLook::StartLine }),
            Inst::EmptyLook(InstEmptyLook { goto: 9, look: EmptyLook::EndLine }),
            Inst::EmptyLook(InstEmptyLook { goto: 10, look: EmptyLook::WordBoundary }),
            Inst::Match(2),
        ],
        matches: vec![2],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let mut empty_flags = EmptyFlags { word_boundary: false, ..Default::default() };
    let mut sparse_set = SparseSet::new(30);
    let ip = 0;

    let mut fsm = Fsm { prog: &prog, start: 0, at: 0, quit_after_match: false, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    fsm.follow_epsilons(ip, &mut sparse_set, empty_flags);
}

