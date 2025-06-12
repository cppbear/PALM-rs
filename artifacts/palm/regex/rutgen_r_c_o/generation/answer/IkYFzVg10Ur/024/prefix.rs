// Answer 0

#[test]
fn test_choose_match_type_with_hint_nfa() {
    let nfa = Program {
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
    
    let suffixes = LiteralSearcher::suffixes(Literals::empty());
    
    let exec_read_only = ExecReadOnly {
        res: vec![String::from("a"), String::from("b")],
        nfa,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
} 

#[test]
fn test_choose_match_type_multiple_regexes_with_suffixes() {
    let nfa = Program {
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

    let suffixes = LiteralSearcher::suffixes(Literals::empty());
    
    let exec_read_only = ExecReadOnly {
        res: vec![String::from("example1"), String::from("example2")],
        nfa,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

#[test]
fn test_choose_match_type_suffixes_complete() {
    let nfa = Program {
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

    let suffixes = LiteralSearcher::suffixes(Literals::empty());
    
    let exec_read_only = ExecReadOnly {
        res: vec![String::from("only_one")],
        nfa,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

