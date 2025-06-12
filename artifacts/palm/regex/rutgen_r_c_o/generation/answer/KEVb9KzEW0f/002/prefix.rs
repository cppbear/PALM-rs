// Answer 0

#[test]
fn test_continue_past_first_match_len_2() {
    let matches = vec![0, 1]; // matches.len() = 2
    let prog = Program {
        insts: vec![],
        matches,
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
    let cache = ProgramCache::default();
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };
    fsm.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_len_3() {
    let matches = vec![0, 1, 2]; // matches.len() = 3
    let prog = Program {
        insts: vec![],
        matches,
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
    let cache = ProgramCache::default();
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };
    fsm.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_len_10() {
    let matches = (0..10).collect::<Vec<_>>(); // matches.len() = 10
    let prog = Program {
        insts: vec![],
        matches,
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
    let cache = ProgramCache::default();
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };
    fsm.continue_past_first_match();
}

#[test]
fn test_continue_past_first_match_len_max() {
    let matches = (0..u32::MAX as usize).collect::<Vec<_>>(); // matches.len() = MAX
    let prog = Program {
        insts: vec![],
        matches,
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
    let cache = ProgramCache::default();
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };
    fsm.continue_past_first_match();
}

