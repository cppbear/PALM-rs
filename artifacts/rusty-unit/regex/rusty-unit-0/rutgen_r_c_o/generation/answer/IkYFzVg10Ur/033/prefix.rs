// Answer 0

#[test]
fn test_choose_match_type_with_nfa_hint_and_suffix_scan() {
    let nfa_program = Program {
        insts: vec![Inst::Match(0), Inst::Split(InstSplit)],
        matches: vec![InstPtr::default()],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1,
    };

    let dfa_program = Program {
        insts: vec![Inst::Char(InstChar)],
        matches: vec![InstPtr::default()],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1,
    };

    let suffixes = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![0]),
        lcs: FreqyPacked::new(vec![0]),
        matcher: Matcher::Empty,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["example_regex".to_string()],
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_program,
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _result = exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

