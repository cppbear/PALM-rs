// Answer 0

#[test]
fn test_fmt_with_empty_instructions() {
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "");
}

#[test]
fn test_fmt_with_single_match() {
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 Match(0)\n (start)\n");
}

#[test]
fn test_fmt_with_save_instruction() {
    let program = Program {
        insts: vec![Inst::Save(InstSave { goto: 1, slot: 0 })],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 Save(0) (goto: 1)\n (start)\n");
}

#[test]
fn test_fmt_with_split_instruction() {
    let program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 1, goto2: 2 })],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 Split(1, 2)\n (start)\n");
}

#[test]
fn test_fmt_with_character_instruction() {
    let program = Program {
        insts: vec![Inst::Char(InstChar { goto: 1, c: 'a' })],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 'a' (goto: 1)\n (start)\n");
}

#[test]
fn test_fmt_with_empty_look_instruction() {
    let program = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartLine })],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 StartLine (goto: 1)\n (start)\n");
}

#[test]
fn test_fmt_with_ranges_instruction() {
    let program = Program {
        insts: vec![Inst::Ranges(InstRanges { goto: 1, ranges: vec![('a', 'z')] })],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 a-z (goto: 1)\n (start)\n");
}

#[test]
fn test_fmt_with_bytes_instruction() {
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: b'a', end: b'z' })],
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
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked {}, lcs: FreqyPacked {}, matcher: Matcher {} },
        dfa_size_limit: 0,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
    assert_eq!(output, "0000 Bytes('a', 'z') (goto: 1)\n (start)\n");
}

