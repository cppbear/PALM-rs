// Answer 0

#[test]
fn test_program_fmt_ranges_with_invalid_goto() {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    let insts = vec![
        Inst::Match(0),
        Inst::Save(InstSave { goto: 2, slot: 1 }),
        Inst::Ranges(InstRanges { goto: 5, ranges: vec![('a', 'z')] }),
        Inst::Ranges(InstRanges { goto: 6, ranges: vec![('0', '9')] }),
    ];
    let captures = vec![None, Some("group1".to_string())];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts,
        matches: vec![0],
        captures,
        capture_name_idx,
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
            complete: true,
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let result = program.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

#[test]
fn test_program_fmt_ranges_with_same_goto() {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    let insts = vec![
        Inst::Match(0),
        Inst::Save(InstSave { goto: 2, slot: 1 }),
        Inst::Ranges(InstRanges { goto: 2, ranges: vec![('a', 'b')] }),
        Inst::Ranges(InstRanges { goto: 2, ranges: vec![('c', 'd')] }),
    ];
    let captures = vec![None, Some("group2".to_string())];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts,
        matches: vec![0],
        captures,
        capture_name_idx,
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
            complete: true,
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 0,
    };

    let result = program.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

