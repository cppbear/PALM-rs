// Answer 0

#[test]
fn test_fmt_empty_program() {
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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let result = format!("{:?}", program);
    assert_eq!(result, ""); // Expect an empty string for an empty program
}

#[test]
fn test_fmt_with_char_instruction() {
    let inst_char = Inst::Char(InstChar {
        goto: 1,
        c: 'a',
    });
    let program = Program {
        insts: vec![inst_char],
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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let result = format!("{:?}", program);
    assert_eq!(result, "000 Char('a') (goto: 1)\n");
}

#[test]
#[should_panic]
fn test_fmt_with_char_instruction_should_panic() {
    let inst_char = Inst::Char(InstChar {
        goto: 0, // Setting goto to 0 to trigger a panic in the `with_goto` function
        c: 'b',
    });
    let program = Program {
        insts: vec![inst_char],
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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let _ = format!("{:?}", program); // This should induce a panic
}

