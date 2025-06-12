// Answer 0

#[test]
fn test_can_exec_dfa_size_limit_zero() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    assert_eq!(can_exec(&program), false);
}

#[test]
fn test_can_exec_exceeds_max_size() {
    let program = Program {
        insts: repeat(Inst::Match(0)).take(::std::i32::MAX as usize + 1).collect(),
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1,
    };

    assert_eq!(can_exec(&program), false);
}

