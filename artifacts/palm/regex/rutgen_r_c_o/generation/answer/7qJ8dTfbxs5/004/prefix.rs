// Answer 0

#[test]
fn test_follow_epsilons_with_ranges() {
    use std::collections::HashMap;

    let mut prog = Program {
        insts: vec![Inst::Ranges(InstRanges {}), Inst::Char(InstChar {})],
        matches: vec![],
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
        prefixes: LiteralSearcher {},
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {},
        states: vec![],
        start_states: vec![],
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = SparseSet::new(STATE_MAX as usize);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    fsm.follow_epsilons(1, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_with_character() {
    use std::collections::HashMap;

    let mut prog = Program {
        insts: vec![Inst::Char(InstChar {}), Inst::Ranges(InstRanges {})],
        matches: vec![],
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
        prefixes: LiteralSearcher {},
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {},
        states: vec![],
        start_states: vec![],
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = SparseSet::new(STATE_MAX as usize);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    fsm.follow_epsilons(0, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_multiple_states() {
    use std::collections::HashMap;

    let mut prog = Program {
        insts: vec![
            Inst::Ranges(InstRanges {}),
            Inst::Ranges(InstRanges {}),
            Inst::Char(InstChar {}),
        ],
        matches: vec![],
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
        prefixes: LiteralSearcher {},
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions {},
        states: vec![],
        start_states: vec![],
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = SparseSet::new(STATE_MAX as usize);
    let flags = EmptyFlags { start: false, end: false, start_line: false, end_line: false, word_boundary: false, not_word_boundary: false };

    fsm.follow_epsilons(0, &mut sparse_set, flags);
}

