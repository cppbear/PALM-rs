// Answer 0

#[test]
fn test_choose_match_type_hint_nfa_with_non_empty_nfa_and_multiple_res() {
    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string(), "example".to_string()],
        nfa: nfa_program,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::Nothing,
    };

    let hint = Some(MatchType::Nfa(MatchNfaType::Auto));
    let _ = exec_read_only.choose_match_type(hint);
}

#[test]
fn test_choose_match_type_hint_nfa_with_non_empty_nfa_and_single_res() {
    let nfa_program = Program {
        insts: vec![Inst::Match(1)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::prefixes(Literals::empty()),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: nfa_program,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::suffixes(Literals::empty()),
        match_type: MatchType::Nothing,
    };

    let hint = Some(MatchType::Nfa(MatchNfaType::Auto));
    let _ = exec_read_only.choose_match_type(hint);
}

