// Answer 0

#[test]
fn test_program_fmt_split_case1() {
    let mut captures = HashMap::new();
    captures.insert(String::from("group1"), 0);
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 1, goto2: 2 }),
            Inst::Match(0),
            Inst::Match(1),
        ],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(captures),
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
    program.fmt(&mut output).unwrap();
}

#[test]
fn test_program_fmt_split_case2() {
    let mut captures = HashMap::new();
    captures.insert(String::from("group2"), 1);
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 5, goto2: 10 }),
            Inst::Bytes(InstBytes { goto: 6, start: 5, end: 10 }),
            Inst::Match(1),
            Inst::Save(InstSave { goto: 2, slot: 0 }),
        ],
        matches: vec![1],
        captures: vec![Some(String::from("capture"))],
        capture_name_idx: Arc::new(captures),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher {
            complete: true,
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 100,
    };
    let mut output = String::new();
    program.fmt(&mut output).unwrap();
}

#[test]
fn test_program_fmt_split_case3() {
    let mut captures = HashMap::new();
    captures.insert(String::from("group3"), 2);
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 3, goto2: 4 }),
            Inst::Char(InstChar { goto: 5, c: 'a' }),
            Inst::EmptyLook(InstEmptyLook { goto: 6, look: EmptyLook::StartLine }),
        ],
        matches: vec![0],
        captures: vec![None, None],
        capture_name_idx: Arc::new(captures),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: true,
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
#[should_panic]
fn test_program_fmt_should_panic_on_error() {
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 100, goto2: 110 }),
        ],
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
    program.fmt(&mut output).unwrap(); // The panic should be triggered due to invalid `goto` values.
}

