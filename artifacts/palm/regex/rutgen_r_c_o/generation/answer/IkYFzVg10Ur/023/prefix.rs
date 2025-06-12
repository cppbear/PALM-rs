// Answer 0

#[test]
fn test_choose_match_type_hint_nfa_non_empty_suffixes_complete() {
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
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::suffixes(Literals::empty());

    let exec = ExecReadOnly {
        res: vec!["test1".to_string(), "test2".to_string()],
        nfa: nfa_program,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _ = exec.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_hint_nfa_non_empty_suffixes_complete_multi_regex() {
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
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::suffixes(Literals::empty());

    let exec = ExecReadOnly {
        res: vec!["regex1".to_string(), "regex2".to_string(), "regex3".to_string()],
        nfa: nfa_program,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _ = exec.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_hint_nfa_non_empty_suffixes_complete_large() {
    let mut regexes = Vec::new();
    for i in 0..100 {
        regexes.push(format!("regex{}", i));
    }

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
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::suffixes(Literals::empty());

    let exec = ExecReadOnly {
        res: regexes,
        nfa: nfa_program,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _ = exec.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

