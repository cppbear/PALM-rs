// Answer 0

#[test]
fn test_program_fmt_ranges_empty() {
    let ranges: Vec<(char, char)> = Vec::new();
    let insts = vec![Inst::Ranges(InstRanges { goto: 1, ranges })];
    let program = Program {
        insts,
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_fmt_ranges_single_character() {
    let ranges: Vec<(char, char)> = vec![('a', 'b')];
    let insts = vec![Inst::Ranges(InstRanges { goto: 1, ranges })];
    let program = Program {
        insts,
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_fmt_ranges_multiple_ranges() {
    let ranges: Vec<(char, char)> = vec![('a', 'z'), ('A', 'Z')];
    let insts = vec![Inst::Ranges(InstRanges { goto: 1, ranges })];
    let program = Program {
        insts,
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
fn test_program_fmt_ranges_large_character_set() {
    let ranges: Vec<(char, char)> = (0..255).map(|c| (char::from(c), char::from(c))).collect();
    let insts = vec![Inst::Ranges(InstRanges { goto: 1, ranges })];
    let program = Program {
        insts,
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

#[test]
#[should_panic]
fn test_program_fmt_invalid_range() {
    let ranges: Vec<(char, char)> = vec![('z', 'a')]; // Invalid range
    let insts = vec![Inst::Ranges(InstRanges { goto: 1, ranges })];
    let program = Program {
        insts,
        matches: vec![0],
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::default(), lcs: FreqyPacked::default(), matcher: Matcher::default() },
        dfa_size_limit: 0,
    };
    let _ = format!("{:?}", program);
}

