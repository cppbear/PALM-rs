// Answer 0

#[test]
fn test_program_with_no_instructions() {
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
            lcp: FreqyPacked {},
            lcs: FreqyPacked {},
            matcher: Matcher {},
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_single_match_instruction() {
    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![Some("group1".to_string())],
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
            complete: true,
            lcp: FreqyPacked {},
            lcs: FreqyPacked {},
            matcher: Matcher {},
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_various_instructions() {
    let program = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Save(InstSave { goto: 2, slot: 0 }),
            Inst::Split(InstSplit { goto1: 3, goto2: 4 }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![Some("group1".to_string()), None],
        capture_name_idx: Arc::new(HashMap::from([(String::from("group1"), 0)])),
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
            lcp: FreqyPacked {},
            lcs: FreqyPacked {},
            matcher: Matcher {},
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_reverse_instruction() {
    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 0b00000001, end: 0b00000010 }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::WordBoundary }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher {
            complete: true,
            lcp: FreqyPacked {},
            lcs: FreqyPacked {},
            matcher: Matcher {},
        },
        dfa_size_limit: 10,
    };
    let _ = format!("{:?}", program);
}

