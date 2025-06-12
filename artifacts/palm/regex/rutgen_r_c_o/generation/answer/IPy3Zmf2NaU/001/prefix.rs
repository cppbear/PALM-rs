// Answer 0

#[test]
fn test_new_empty_cache_with_empty_insts_and_zero_byte_classes() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
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
    let _cache = Cache::new(&prog);
}

#[test]
fn test_new_empty_cache_with_single_inst_and_max_byte_classes() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![Some("group".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256].into_iter().map(|_| 254).collect(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 255,
    };
    let _cache = Cache::new(&prog);
}

#[test]
fn test_new_empty_cache_with_multiple_insts_and_mid_byte_classes() {
    let prog = Program {
        insts: vec![Inst::Save(InstSave::new()), Inst::Split(InstSplit::new())],
        matches: vec![1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 1,
        byte_classes: vec![1, 2, 3, 4, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 254],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };
    let _cache = Cache::new(&prog);
}

#[test]
fn test_new_empty_cache_with_max_insts_and_empty_byte_classes() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar::new())],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 20,
    };
    let _cache = Cache::new(&prog);
}

#[test]
fn test_new_empty_cache_with_no_insts_and_one_byte_class() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![1],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 5,
    };
    let _cache = Cache::new(&prog);
}

