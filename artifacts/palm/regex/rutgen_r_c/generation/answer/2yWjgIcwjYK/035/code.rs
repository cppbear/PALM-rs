// Answer 0

fn test_program_fmt_with_save() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::Write;

    let capture_name_idx = Arc::new(HashMap::new());

    let program = Program {
        insts: vec![
            Inst::Match(0),
            Inst::Save(InstSave { goto: 2, slot: 0 }),
            Inst::Save(InstSave { goto: 3, slot: 1 }),
        ],
        matches: vec![0],
        captures: vec![None, Some("group1".to_string())],
        capture_name_idx: capture_name_idx.clone(),
        start: 2,
        byte_classes: vec![1, 2],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::new(), lcs: FreqyPacked::new(), matcher: Matcher::new() },
        dfa_size_limit: 0,
    };

    let mut output = String::new();
    let result = program.fmt(&mut write!(output, ""));

    assert!(result.is_ok());
    assert!(!output.contains("(start)"));  // Ensuring start isn't printed
    assert!(output.contains("000 Match(0)")); // First instruction
    assert!(output.contains("001 Save(0) (goto: 2)")); // Second instruction
    assert!(output.contains("002 Save(1) (goto: 3)")); // Third instruction
}

fn test_program_fmt_with_invalid_write() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::fmt::Write;

    let capture_name_idx = Arc::new(HashMap::new());

    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
        ],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: capture_name_idx.clone(),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher { complete: false, lcp: FreqyPacked::new(), lcs: FreqyPacked::new(), matcher: Matcher::new() },
        dfa_size_limit: 0,
    };

    let mut output = String::new();
    let result = program.fmt(&mut write!(output, "").unwrap_err());

    assert!(result.is_err());  // Ensuring the result is an error
}

