// Answer 0

#[test]
fn test_follow_epsilons_valid_input_case_1() {
    let mut prog = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::EndLine })],
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
        dfa_size_limit: 10,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
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
    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: true, start_line: false, end_line: true, word_boundary: false, not_word_boundary: false };
    fsm.cache.stack.push(0);
    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_valid_input_case_2() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::EndLine }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::StartLine }),
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
        dfa_size_limit: 10,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
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
    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: true, start_line: false, end_line: true, word_boundary: false, not_word_boundary: false };
    fsm.cache.stack.push(0);
    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_multiple_states() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::EndLine }),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartText }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::EndText }),
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
        dfa_size_limit: 10,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
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
    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: true, start_line: false, end_line: true, word_boundary: false, not_word_boundary: false };
    fsm.cache.stack.push(0);
    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
#[should_panic]
fn test_follow_epsilons_invalid_ip() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::EndLine }),
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
        dfa_size_limit: 10,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
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
    let mut q = SparseSet::new(10);
    let flags = EmptyFlags { start: false, end: true, start_line: false, end_line: true, word_boundary: false, not_word_boundary: false };
    fsm.cache.stack.push(2); // Invalid index
    fsm.follow_epsilons(2, &mut q, flags);
}

