// Answer 0

#[test]
fn test_fmt_empty_program() {
    use std::fmt::Write;
    
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

    let mut output = String::new();
    assert!(program.fmt(&mut output).is_ok());
    assert_eq!(output, "");
}

#[test]
fn test_fmt_program_with_single_match() {
    use std::fmt::Write;

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
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let mut output = String::new();
    assert!(program.fmt(&mut output).is_ok());
    assert_eq!(output, "0000 Match(0)\n (start)\n");
}

#[test]
fn test_fmt_program_with_multiple_instructions() {
    use std::fmt::Write;

    let program = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Save(InstSave { goto: 2, slot: 0 }),
            Inst::Match(1),
        ],
        matches: vec![1],
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

    let mut output = String::new();
    assert!(program.fmt(&mut output).is_ok());
    assert_eq!(output, "0000 'a' (goto: 1)\n0001 Save(0) (goto: 2)\n0002 Match(1)\n (start)\n");
}

