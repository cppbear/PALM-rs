// Answer 0

fn test_program_fmt_with_ranges() -> fmt::Result {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    let insts = vec![
        Inst::Ranges(InstRanges {
            goto: 2,
            ranges: vec![('a', 'z'), ('A', 'Z')],
        }),
        Inst::Match(0),
        Inst::Save(InstSave { goto: 3, slot: 0 }),
        Inst::Char(InstChar { goto: 4, c: 'b' }),
    ];
    
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![None],
        capture_name_idx,
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
        dfa_size_limit: 10,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", program);
    
    assert!(result.is_ok());
    assert!(!output.contains(" (start)"));
    assert!(result.is_ok());
    
    let last_newline_result = write!(&mut output, "\n");
    
    assert!(last_newline_result.is_ok());
    
    Ok(result.unwrap())
}

fn test_program_fmt_with_non_ranges() -> fmt::Result {
    let insts = vec![
        Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::StartLine }),
        Inst::Match(0),
        Inst::Bytes(InstBytes { goto: 3, start: 0, end: 255 }),
    ];

    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![None],
        capture_name_idx,
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
        dfa_size_limit: 10,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", program);
    
    assert!(result.is_ok());
    assert!(!output.contains(" (start)"));
    assert!(result.is_ok());
    
    Ok(result.unwrap())
}

