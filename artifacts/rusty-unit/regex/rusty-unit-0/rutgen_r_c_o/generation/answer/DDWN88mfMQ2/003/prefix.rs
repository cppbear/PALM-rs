// Answer 0

#[test]
fn test_reverse_no_match_at_zero() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    let cache = ProgramCache {
        dfa_reverse: ProgramCacheInner::default(),
        ..Default::default()
    };
    let text = &[0];
    let at = 0;
    reverse(&prog, &cache, false, text, at);
}

#[test]
fn test_reverse_no_match_at_one() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    let mut cache = ProgramCache {
        dfa_reverse: ProgramCacheInner::default(),
        ..Default::default()
    };
    let text = &[1];
    let at = 1;
    reverse(&prog, &cache, false, text, at);
}

#[test]
fn test_reverse_no_match_at_two() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    let mut cache = ProgramCache {
        dfa_reverse: ProgramCacheInner::default(),
        ..Default::default()
    };
    let text = &[2];
    let at = 2;
    reverse(&prog, &cache, false, text, at);
}

#[test]
fn test_reverse_no_match_at_three() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    let mut cache = ProgramCache {
        dfa_reverse: ProgramCacheInner::default(),
        ..Default::default()
    };
    let text = &[3];
    let at = 3;
    reverse(&prog, &cache, false, text, at);
}

#[test]
fn test_reverse_no_match_at_four() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };
    let mut cache = ProgramCache {
        dfa_reverse: ProgramCacheInner::default(),
        ..Default::default()
    };
    let text = &[4];
    let at = 4;
    reverse(&prog, &cache, false, text, at);
}

