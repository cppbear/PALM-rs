// Answer 0

#[test]
fn test_start_state_all_flags_set() {
    let mut sparse_set = SparseSet::new(8);
    sparse_set.insert(1);
    
    let empty_flags = EmptyFlags {
        start: true,
        end: true,
        start_line: true,
        end_line: true,
        word_boundary: true,
        not_word_boundary: true,
    };
    
    let mut state_flags = StateFlags(0b11111111);

    let program = Program {
        insts: vec![], 
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
        dfa_size_limit: 16,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.cache.start_states[flagi] = 2;

    let result = fsm.start_state(&mut sparse_set, empty_flags, state_flags);
}

#[test]
fn test_start_state_no_flags_set() {
    let mut sparse_set = SparseSet::new(8);
    
    let empty_flags = EmptyFlags::default();
    let mut state_flags = StateFlags(0);

    let program = Program {
        insts: vec![], 
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
        dfa_size_limit: 16,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.cache.start_states[flagi] = 3;

    let result = fsm.start_state(&mut sparse_set, empty_flags, state_flags);
}

#[test]
fn test_start_state_partial_flags() {
    let mut sparse_set = SparseSet::new(8);
    sparse_set.insert(1);
    
    let empty_flags = EmptyFlags {
        start: false,
        end: true,
        start_line: false,
        end_line: false,
        word_boundary: true,
        not_word_boundary: false,
    };
    
    let mut state_flags = StateFlags(0b00000001);

    let program = Program {
        insts: vec![], 
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
        dfa_size_limit: 16,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.cache.start_states[flagi] = 1;

    let result = fsm.start_state(&mut sparse_set, empty_flags, state_flags);
}

#[test]
fn test_start_state_edge_case_cache_full() {
    let mut sparse_set = SparseSet::new(8);
    
    let empty_flags = EmptyFlags::default();
    let mut state_flags = StateFlags(0);

    let program = Program {
        insts: vec![], 
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
        dfa_size_limit: 16,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![STATE_DEAD; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.cache.start_states[flagi] = 5; 

    let result = fsm.start_state(&mut sparse_set, empty_flags, state_flags);
}

