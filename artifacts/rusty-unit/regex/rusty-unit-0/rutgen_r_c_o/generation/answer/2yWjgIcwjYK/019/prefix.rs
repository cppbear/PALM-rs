// Answer 0

#[test]
fn test_empty_look_start_line() {
    let inst = Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartLine });
    let program = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 10,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_empty_look_end_line() {
    let inst = Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::EndLine });
    let program = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 10,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_empty_look_word_boundary() {
    let inst = Inst::EmptyLook(InstEmptyLook { goto: 3, look: EmptyLook::WordBoundary });
    let program = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 10,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_empty_look_not_word_boundary() {
    let inst = Inst::EmptyLook(InstEmptyLook { goto: 4, look: EmptyLook::NotWordBoundary });
    let program = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 10,
    };
    let _ = format!("{:?}", program);
}

