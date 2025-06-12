// Answer 0

#[test]
fn test_follow_epsilons_case_1() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State::default(); 5],
        start_states: vec![0; 10],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartLine }),
            Inst::Match(0),
            Inst::EmptyLook(InstEmptyLook { goto: 0, look: EmptyLook::EndLine }),
        ],
        matches: vec![0],
        captures: vec![None],
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
        dfa_size_limit: 100,
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
    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start_line: true,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };
    fsm.follow_epsilons(0, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case_2() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State::default(); 5],
        start_states: vec![0; 10],
        stack: vec![1],
        flush_count: 0,
        size: 0,
    };
    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::EndLine }),
            Inst::Match(1),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartText }),
        ],
        matches: vec![0],
        captures: vec![None],
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
        dfa_size_limit: 100,
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
    let mut q = SparseSet::new(10);
    q.insert(1);
    let flags = EmptyFlags {
        start_line: false,
        end_line: true,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };
    fsm.follow_epsilons(1, &mut q, flags);
}

#[test]
fn test_follow_epsilons_case_3() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State::default(); 5],
        start_states: vec![0; 10],
        stack: vec![2],
        flush_count: 0,
        size: 0,
    };
    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 3, look: EmptyLook::WordBoundary }),
            Inst::Match(1),
            Inst::EmptyLook(InstEmptyLook { goto: 4, look: EmptyLook::NotWordBoundary }),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartText }),
        ],
        matches: vec![0],
        captures: vec![None],
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
        dfa_size_limit: 100,
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
    let mut q = SparseSet::new(10);
    q.insert(2);
    let flags = EmptyFlags {
        start_line: false,
        end_line: false,
        start: true,
        end: false,
        word_boundary: true,
        not_word_boundary: false,
    };
    fsm.follow_epsilons(2, &mut q, flags);
}

