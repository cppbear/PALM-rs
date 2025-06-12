// Answer 0

#[test]
fn test_continue_past_first_match_reverse_true_multiple_matches() {
    let program = Program {
        insts: Vec::new(),
        matches: vec![0, 1],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 512,
    };
    
    let cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };
    
    fsm.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_reverse_true_single_match() {
    let program = Program {
        insts: Vec::new(),
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 512,
    };
    
    let cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };
    
    fsm.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_reverse_false_multiple_matches() {
    let program = Program {
        insts: Vec::new(),
        matches: vec![0, 1],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 512,
    };
    
    let cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };
    
    fsm.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_reverse_false_single_match() {
    let program = Program {
        insts: Vec::new(),
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 512,
    };
    
    let cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: Vec::new(),
        start_states: Vec::new(),
        stack: Vec::new(),
        flush_count: 0,
        size: 0,
    };

    let fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };
    
    fsm.continue_past_first_match();
}

