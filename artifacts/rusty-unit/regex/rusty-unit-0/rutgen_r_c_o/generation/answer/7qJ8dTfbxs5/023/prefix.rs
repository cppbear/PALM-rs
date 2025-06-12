// Answer 0

#[test]
fn test_follow_epsilons_simple() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::default(); 256],
        start_states: vec![0; 256],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
    };

    let mut sparse_set = SparseSet::new(256);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(0, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_with_multiple_saves() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::default(); 256],
        start_states: vec![0; 256],
        stack: vec![0, 1],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 2, slot: 0 }),
            Inst::Save(InstSave { goto: 3, slot: 1 }),
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
    };

    let mut sparse_set = SparseSet::new(256);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(0, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_with_stack_pop() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::default(); 256],
        start_states: vec![0; 256],
        stack: vec![5],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 6, slot: 0 }),
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
    };

    let mut sparse_set = SparseSet::new(256);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(5, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_reroute_no_push() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State::default(); 256],
        start_states: vec![0; 256],
        stack: vec![4],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 5, slot: 0 }),
            Inst::Split(InstSplit { goto1: 6, goto2: 7 }),
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
    };

    let mut sparse_set = SparseSet::new(256);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.follow_epsilons(4, &mut sparse_set, flags);
}

