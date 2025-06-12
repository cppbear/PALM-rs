// Answer 0

#[test]
fn test_can_exec_empty_program() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1, // non-zero
    };

    assert!(can_exec(&program));
}

#[test]
fn test_can_exec_with_max_instruction_length() {
    let max_instructions = ::std::i32::MAX as usize;
    let instructions = vec![Inst::EmptyLook(InstEmptyLook)] // no Char or Ranges instructions
        .into_iter()
        .cycle()
        .take(max_instructions)
        .collect::<Vec<_>>();

    let program = Program {
        insts: instructions,
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1, // non-zero
    };

    assert!(can_exec(&program));
}

