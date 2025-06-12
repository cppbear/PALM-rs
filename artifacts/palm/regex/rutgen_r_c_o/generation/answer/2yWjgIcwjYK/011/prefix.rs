// Answer 0

#[test]
fn test_program_fmt_ranges_valid() {
    let insts = vec![
        Inst::Ranges(InstRanges {
            goto: 2,
            ranges: vec![('a', 'b')],
        }),
        Inst::Match(0),
        Inst::Ranges(InstRanges {
            goto: 4,
            ranges: vec![('c', 'd')],
        }),
        Inst::Save(InstSave { goto: 5, slot: 0 }),
        Inst::EmptyLook(InstEmptyLook { goto: 6, look: EmptyLook::StartLine }),
        Inst::Char(InstChar { goto: 7, c: 'e' }),
        Inst::Split(InstSplit { goto1: 8, goto2: 9 }),
        Inst::Bytes(InstBytes { goto: 10, start: 1, end: 5 }),
        Inst::Ranges(InstRanges {
            goto: 11,
            ranges: vec![('f', 'g'), ('h', 'j')],
        }),
        Inst::Match(1),
    ];
    
    let captures = vec![None; 10];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts,
        matches: vec![0],
        captures,
        capture_name_idx,
        start: 2,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
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
        dfa_size_limit: 10,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", program);
}

#[test]
#[should_panic]
fn test_program_fmt_ranges_invalid_goto() {
    let insts = vec![
        Inst::Ranges(InstRanges {
            goto: 20,
            ranges: vec![('a', 'b')],
        }),
    ];
    
    let captures = vec![None; 1];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts,
        matches: vec![0],
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
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
        dfa_size_limit: 10,
    };

    let _ = program.fmt(&mut String::new());
}

#[test]
fn test_program_fmt_empty_ranges() {
    let insts = vec![
        Inst::Ranges(InstRanges {
            goto: 1,
            ranges: vec![],
        }),
        Inst::Match(0),
    ];
    
    let captures = vec![None; 1];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts,
        matches: vec![0],
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
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
        dfa_size_limit: 10,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", program);
}

#[test]
fn test_program_fmt_multiple_ranges() {
    let insts = vec![
        Inst::Ranges(InstRanges {
            goto: 1,
            ranges: vec![('a', 'c'), ('d', 'f')],
        }),
        Inst::Match(0),
    ];
    
    let captures = vec![None; 1];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts,
        matches: vec![0],
        captures,
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
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
        dfa_size_limit: 10,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", program);
}

