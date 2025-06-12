// Answer 0

#[test]
fn test_deref_empty() {
    let program = Program {
        insts: Vec::new(),
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
    let _ = program.deref();
}

#[test]
fn test_deref_single_inst() {
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
            lcp: FreqyPacked {},
            lcs: FreqyPacked {},
            matcher: Matcher {},
        },
        dfa_size_limit: 1,
    };
    let _ = program.deref();
}

#[test]
fn test_deref_multiple_insts() {
    let insts = vec![
        Inst::Match(0),
        Inst::Save(InstSave {}),
        Inst::Split(InstSplit {}),
        Inst::EmptyLook(InstEmptyLook {}),
        Inst::Char(InstChar {}),
    ];
    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
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
        dfa_size_limit: 100,
    };
    let _ = program.deref();
}

#[test]
fn test_deref_max_size() {
    let insts = (0..10000).map(|i| Inst::Match(i)).collect();
    let program = Program {
        insts,
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
        prefixes: LiteralSearcher {
            complete: true,
            lcp: FreqyPacked {},
            lcs: FreqyPacked {},
            matcher: Matcher {},
        },
        dfa_size_limit: 10,
    };
    let _ = program.deref();
}

