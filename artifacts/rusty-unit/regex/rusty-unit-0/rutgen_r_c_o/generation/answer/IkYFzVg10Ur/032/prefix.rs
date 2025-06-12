// Answer 0

#[test]
fn test_choose_match_type_with_hint_nfa_non_empty_res_len_2_can_exec() {
    let res = vec!["test_regex1".to_string(), "test_regex2".to_string()];
    
    let nfa_insts = vec![Inst::Match(0)];
    let nfa = Program {
        insts: nfa_insts,
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1024,
    };

    let dfa = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1024,
    };

    let suffixes = LiteralSearcher::empty();
    
    let exec_read_only = ExecReadOnly {
        res,
        nfa,
        dfa,
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    let hint = Some(MatchType::Nfa(MatchNfaType::Auto));

    let _ = exec_read_only.choose_match_type(hint);
}

