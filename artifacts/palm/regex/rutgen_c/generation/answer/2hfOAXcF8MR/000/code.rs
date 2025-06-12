// Answer 0

#[test]
fn test_deref_empty() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };

    assert_eq!(program.deref().len(), 0);
}

#[test]
fn test_deref_single_instruction() {
    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };

    assert_eq!(program.deref().len(), 1);
}

#[test]
fn test_deref_multiple_instructions() {
    let program = Program {
        insts: vec![Inst::Char(InstChar), Inst::Save(InstSave)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };

    assert_eq!(program.deref().len(), 2);
}

