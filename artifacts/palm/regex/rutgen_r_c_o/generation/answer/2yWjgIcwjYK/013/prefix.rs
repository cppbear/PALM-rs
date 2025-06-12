// Answer 0

#[test]
fn test_program_with_char_instruction_goto_zero() {
    let inst = Inst::Char(InstChar { goto: 0, c: 'a' });
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
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_char_instruction_goto_one() {
    let inst = Inst::Char(InstChar { goto: 1, c: 'b' });
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
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_char_instruction_goto_two() {
    let inst = Inst::Char(InstChar { goto: 2, c: 'c' });
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
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_char_instruction_goto_three() {
    let inst = Inst::Char(InstChar { goto: 3, c: 'd' });
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
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_with_char_instruction_goto_four() {
    let inst = Inst::Char(InstChar { goto: 4, c: 'e' });
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
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::new(),
            lcs: FreqyPacked::new(),
            matcher: Matcher::new(),
        },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

