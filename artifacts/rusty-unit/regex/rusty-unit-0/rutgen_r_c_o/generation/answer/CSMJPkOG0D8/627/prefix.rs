// Answer 0

#[test]
fn test_exec_at_empty_input() {
    let program = Program {
        insts: vec![],
        matches: vec![],
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
        dfa_size_limit: 0,
    };

    let cache = ProgramCache;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let text: Vec<u8> = vec![];
    fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_single_character_no_match() {
    let program = Program {
        insts: vec![],
        matches: vec![],
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
        dfa_size_limit: 0,
    };

    let cache = ProgramCache;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let text: Vec<u8> = vec![b'a'];
    fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_single_character_match_state_dead() {
    let program = Program {
        insts: vec![],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let cache = ProgramCache;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_DEAD,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let text: Vec<u8> = vec![b'b'];
    fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_eof_no_match() {
    let program = Program {
        insts: vec![],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let cache = ProgramCache;
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_UNKNOWN,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let text: Vec<u8> = vec![b'a'];
    fsm.exec_at(&mut qcur, &mut qnext, &text);
}

