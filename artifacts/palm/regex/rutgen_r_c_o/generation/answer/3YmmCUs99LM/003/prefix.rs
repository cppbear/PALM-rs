// Answer 0

#[test]
fn test_forward_no_match_at_start() {
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
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    let cache = ProgramCache::new();
    let text: &[u8] = &[0, 0, 0, 0, 0, 0];
    let at = 0;
    Fsm::forward(&prog, &cache, false, text, at);
}

#[test]
fn test_forward_no_match_at_specific_index() {
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
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    let cache = ProgramCache::new();
    let text: &[u8] = &[1, 2];
    let at = 1;
    Fsm::forward(&prog, &cache, false, text, at);
}

#[test]
fn test_forward_no_match_single_byte() {
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
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    let cache = ProgramCache::new();
    let text: &[u8] = &[3];
    let at = 0;
    Fsm::forward(&prog, &cache, false, text, at);
}

#[test]
fn test_forward_no_match_text_exceeds_index() {
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
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    let cache = ProgramCache::new();
    let text: &[u8] = &[0, 1, 2, 3, 4, 5];
    let at = 5;
    Fsm::forward(&prog, &cache, false, text, at);
}

