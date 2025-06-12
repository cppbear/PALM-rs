// Answer 0

#[test]
fn test_fmt_match_start() {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);

    let program = Program {
        insts: vec![
            Inst::Match(0),
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::Split(InstSplit { goto1: 1, goto2: 2 }),
        ],
        matches: vec![0],
        captures: vec![Some("group1".to_string())],
        capture_name_idx: Arc::new(capture_name_idx),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 128,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", program);

    assert!(result.is_ok());
    let expected_output = "0000 Match(0) (start)\n0001 Save(0)\n0002 Split(1, 2)\n";
    assert_eq!(output, expected_output);
}

#[test]
fn test_fmt_with_multiple_inst() {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group2".to_string(), 0);

    let program = Program {
        insts: vec![
            Inst::Match(0),
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::Char(InstChar { goto: 2, c: 'a' }),
        ],
        matches: vec![0],
        captures: vec![Some("group2".to_string())],
        capture_name_idx: Arc::new(capture_name_idx),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher {
            complete: false,
            lcp: FreqyPacked::default(),
            lcs: FreqyPacked::default(),
            matcher: Matcher::default(),
        },
        dfa_size_limit: 128,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", program);

    assert!(result.is_ok());
    let expected_output = "0000 Match(0) (start)\n0001 Save(0)\n0002 'a'\n";
    assert_eq!(output, expected_output);
}

