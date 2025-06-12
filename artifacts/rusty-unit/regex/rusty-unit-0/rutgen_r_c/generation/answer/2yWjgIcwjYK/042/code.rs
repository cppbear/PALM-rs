// Answer 0

#[test]
fn test_program_fmt_single_match() {
    let match_inst = Inst::Match(0);
    let program = Program {
        insts: vec![match_inst.clone()],
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
    let result = program.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "000 Match(0)\n");
}

#[test]
fn test_program_fmt_multiple_instructions() {
    let program = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Match(0)
        ],
        matches: vec![1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 1,
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
    let result = program.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "00 'a' (goto: 01)\n01 Match(0) (start)\n");
}

#[test]
fn test_program_fmt_split_instruction() {
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 1, goto2: 2 }),
            Inst::Match(0),
            Inst::EmptyLook(InstEmptyLook { goto: 3, look: EmptyLook::WordBoundary }),
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
    let result = program.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "00 Split(01, 02)\n01 Match(0)\n02 EmptyLook(WordBoundary)\n");
}

