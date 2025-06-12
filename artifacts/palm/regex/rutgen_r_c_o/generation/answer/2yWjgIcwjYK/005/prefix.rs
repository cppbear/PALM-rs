// Answer 0

#[test]
fn test_program_fmt_case_1() {
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 0b0000_0001, end: 0b0000_0010 })],
        matches: vec![0],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
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
    program.fmt(&mut output).unwrap();
}

#[test]
fn test_program_fmt_case_2() {
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 2, start: 0b0000_0011, end: 0b0000_0100 }),
            Inst::Bytes(InstBytes { goto: 3, start: 0b0000_0101, end: 0b0000_0110 })
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
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
    program.fmt(&mut output).unwrap();
}

#[test]
fn test_program_fmt_case_3() {
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 0b0000_0111, end: 0b0000_1000 }),
            Inst::Bytes(InstBytes { goto: 3, start: 0b0000_1001, end: 0b0000_1010 }),
            Inst::Bytes(InstBytes { goto: 4, start: 0b0000_1011, end: 0b0000_1100 }),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
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
    program.fmt(&mut output).unwrap();
}

#[test]
fn test_program_fmt_case_4() {
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 2, start: 0b0000_1101, end: 0b0000_1110 }),
            Inst::Bytes(InstBytes { goto: 3, start: 0b0000_1111, end: 0b0001_0000 }),
            Inst::Bytes(InstBytes { goto: 1, start: 0b0001_0001, end: 0b0001_0010 }),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
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
    program.fmt(&mut output).unwrap();
}

