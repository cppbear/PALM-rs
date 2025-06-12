// Answer 0

fn test_program_with_split_instruction() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("test".to_string(), 0);
    let capture_name_idx_arc = Arc::new(capture_name_idx);
    
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 2, goto2: 3 }),
            Inst::Save(InstSave { goto: 4, slot: 1 }),
            Inst::Match(0),
            Inst::Char(InstChar { goto: 5, c: 'a' }),
        ],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: capture_name_idx_arc,
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: Default::default(), lcs: Default::default(), matcher: Default::default() },
        dfa_size_limit: 100,
    };

    let output = format!("{:?}", program);

    assert!(output.contains("Split(2, 3)"));
    assert!(output.contains(" (start)"));
}

fn test_program_with_multiple_instructions() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("example".to_string(), 0);
    let capture_name_idx_arc = Arc::new(capture_name_idx);
    
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 3, goto2: 4 }),
            Inst::EmptyLook(InstEmptyLook { goto: 5, look: EmptyLook::WordBoundary }),
            Inst::Char(InstChar { goto: 6, c: 'b' }),
            Inst::Match(1),
            Inst::Save(InstSave { goto: 7, slot: 2 }),
        ],
        matches: vec![1],
        captures: vec![None],
        capture_name_idx: capture_name_idx_arc,
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: Default::default(), lcs: Default::default(), matcher: Default::default() },
        dfa_size_limit: 150,
    };

    let output = format!("{:?}", program);

    assert!(output.contains("Split(3, 4)"));
    assert!(output.contains("WordBoundary"));
    assert!(output.contains(" (start)"));
}

