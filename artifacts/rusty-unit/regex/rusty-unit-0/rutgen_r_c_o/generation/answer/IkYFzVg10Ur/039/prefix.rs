// Answer 0

#[test]
fn test_choose_match_type_with_nfa_hint_and_non_empty_nfa() {
    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1024,
    };

    let suffixes_searcher = LiteralSearcher::prefixes(Literals::empty());
    
    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: nfa_program,
        dfa: Program {
            insts: vec![],
            matches: vec![],
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
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 1024,
        },
        dfa_reverse: Program::default(),
        suffixes: suffixes_searcher,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _ = exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_with_nfa_hint_and_complete_suffixes() {
    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1024,
    };

    let suffixes_searcher = LiteralSearcher::suffixes(Literals::empty());

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: nfa_program,
        dfa: Program {
            insts: vec![],
            matches: vec![],
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
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 1024,
        },
        dfa_reverse: Program::default(),
        suffixes: suffixes_searcher,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _ = exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_with_nfa_hint_and_non_anchored() {
    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1024,
    };

    let suffixes_searcher = LiteralSearcher::prefixes(Literals::empty());

    let exec_read_only = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: nfa_program,
        dfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: false,
            is_bytes: true,
            is_dfa: true,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 1024,
        },
        dfa_reverse: Program::default(),
        suffixes: suffixes_searcher,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _ = exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

