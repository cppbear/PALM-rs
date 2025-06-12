// Answer 0

#[test]
fn test_cached_state_key_char_instruction() {
    let mut state_flags = StateFlags(0);
    let program = Program {
        insts: vec![Inst::Char(InstChar { c: 'a' as u8 }), Inst::Match(0)],
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
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::default(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };
    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };
    let _ = fsm.cached_state_key(&sparse_set, &mut state_flags);
}

#[test]
fn test_cached_state_key_ranges_instruction() {
    let mut state_flags = StateFlags(0);
    let program = Program {
        insts: vec![Inst::Ranges(InstRanges { ranges: vec![] }), Inst::Match(0)],
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
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::default(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };
    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };
    let _ = fsm.cached_state_key(&sparse_set, &mut state_flags);
}

#[test]
fn test_cached_state_key_multiple_states() {
    let mut state_flags = StateFlags(1);
    let program = Program {
        insts: vec![
            Inst::Char(InstChar { c: 'a' as u8 }),
            Inst::Char(InstChar { c: 'b' as u8 }),
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
        dfa_size_limit: 1024,
    };
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::default(),
            states: vec![],
            start_states: vec![],
            stack: vec![],
            flush_count: 0,
            size: 0,
        },
    };
    let sparse_set = SparseSet {
        dense: vec![0, 1],
        sparse: vec![0, 1],
        size: 2,
    };
    let _ = fsm.cached_state_key(&sparse_set, &mut state_flags);
}

