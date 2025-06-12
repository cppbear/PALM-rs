// Answer 0

#[test]
fn test_exec_at_case_1() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: STATE_MAX,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_MAX,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet::default();
    let mut qnext = SparseSet::default();
    let text = vec![0; STATE_MAX];

    let _result = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_case_2() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: STATE_MAX - 1,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 2048,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_MAX - 1,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet::default();
    let mut qnext = SparseSet::default();
    let text = vec![0; STATE_MAX];

    let _result = fsm.exec_at(&mut qcur, &mut qnext, &text);
} 

#[test]
fn test_exec_at_case_3() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: STATE_MAX - 2,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 512,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_MAX - 2,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet::default();
    let mut qnext = SparseSet::default();
    let text = vec![1; STATE_MAX];

    let _result = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_case_4() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![1],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: STATE_MAX - 1,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 256,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_MAX - 1,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet::default();
    let mut qnext = SparseSet::default();
    let text = vec![0; STATE_MAX];

    let _result = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

