// Answer 0

#[test]
fn test_cache_new_empty_program() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };
    let _cache = Cache::new(&prog);
}

#[test]
fn test_cache_new_program_with_single_instruction() {
    let prog = Program {
        insts: vec![Inst::new()],
        matches: vec![InstPtr(0)],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(HashMap::from([("group1".to_string(), 0)])),
        start: InstPtr(0),
        byte_classes: vec![0, 1],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1,
    };
    let _cache = Cache::new(&prog);
}

#[test]
fn test_cache_new_program_with_multiple_instructions() {
    let prog = Program {
        insts: vec![Inst::new(), Inst::new(), Inst::new()],
        matches: vec![InstPtr(0)],
        captures: vec![Some("group1".to_string()), Some("group2".to_string())],
        capture_name_idx: Arc::new(HashMap::from([
            ("group1".to_string(), 0),
            ("group2".to_string(), 1),
        ])),
        start: InstPtr(0),
        byte_classes: vec![0, 1, 2],
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 5,
    };
    let _cache = Cache::new(&prog);
}

#[test]
fn test_cache_new_large_program() {
    let mut insts = Vec::new();
    for _ in 0..1000 {
        insts.push(Inst::new());
    }
    let prog = Program {
        insts,
        matches: vec![InstPtr(0)],
        captures: (0..100).map(|i| Some(format!("group{}", i))).collect(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: (0..100).map(|i| i as u8).collect(),
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    let _cache = Cache::new(&prog);
}

#[test]
#[should_panic]
fn test_cache_new_invalid_program() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };
    let _cache = Cache::new(&prog);
}

