// Answer 0

#[test]
fn test_can_exec_valid_case() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let capture_name_idx = Arc::new(HashMap::new());

    let insts = vec![
        Inst::Match(0),
    ];

    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![None],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };

    can_exec(&program);
}

#[test]
fn test_can_exec_length_limit() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let capture_name_idx = Arc::new(HashMap::new());

    let insts = repeat(Inst::Match(0)).take(::std::i32::MAX as usize).collect::<Vec<_>>();

    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![None],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };

    can_exec(&program);
}

#[test]
fn test_can_exec_with_empty_instructions() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let capture_name_idx = Arc::new(HashMap::new());

    let insts = vec![];

    let program = Program {
        insts,
        matches: vec![0],
        captures: vec![None],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };

    can_exec(&program);
}

#[test]
fn test_can_exec_multiple_match_instructions() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let capture_name_idx = Arc::new(HashMap::new());

    let insts = vec![
        Inst::Match(0),
        Inst::Match(1),
        Inst::Match(2),
    ];

    let program = Program {
        insts,
        matches: vec![0, 1, 2],
        captures: vec![None],
        capture_name_idx,
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };

    can_exec(&program);
}

