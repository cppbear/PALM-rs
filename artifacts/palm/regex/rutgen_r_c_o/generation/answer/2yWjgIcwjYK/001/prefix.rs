// Answer 0

#[test]
fn test_program_with_bytes_instruction_valid_range() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);

    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 97, end: 122 }), // Bytes(a-z)
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(capture_name_idx),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };

    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_bytes_instruction_edge_case() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);

    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 0, end: 0 }), // Bytes(0)
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(capture_name_idx),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };

    let _ = format!("{:?}", program);
}

#[test]
#[should_panic]
fn test_program_with_bytes_instruction_invalid_range() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);

    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 100, end: 50 }), // Invalid byte range
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(capture_name_idx),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };

    let _ = format!("{:?}", program);
}

