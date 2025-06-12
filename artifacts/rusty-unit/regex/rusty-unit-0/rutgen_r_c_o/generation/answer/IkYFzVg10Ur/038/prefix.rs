// Answer 0

#[test]
fn test_choose_match_type_with_nfa_hint() {
    let program = Program {
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
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::empty();
    let readonly = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes,
        match_type: MatchType::Nothing,
    };

    let result = readonly.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_with_empty_nfa_and_suffixes() {
    let program = Program {
        insts: vec![Inst::Match(0); 5],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::suffixes(Literals::empty());
    let readonly = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes,
        match_type: MatchType::Nothing,
    };

    let result = readonly.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_with_res_length_one() {
    let program = Program {
        insts: vec![Inst::Match(0), Inst::Char(InstChar::default())],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::empty();
    let readonly = ExecReadOnly {
        res: vec!["literal".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes,
        match_type: MatchType::Nothing,
    };

    let result = readonly.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_with_dfa_execution_false() {
    let program = Program {
        insts: vec![Inst::Match(0), Inst::Save(InstSave::default())],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::empty();
    let readonly = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: program,
        dfa: program,
        dfa_reverse: program,
        suffixes,
        match_type: MatchType::Nothing,
    };

    let result = readonly.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

