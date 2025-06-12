// Answer 0

#[test]
fn test_should_suffix_scan_case_1() {
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![b"abc".to_vec()]));
    let lcp = FreqyPacked::new(vec![b"a".to_vec()]);
    let dfa_prefixes = LiteralSearcher::prefixes(Literals::new(vec![b"ab".to_vec()]));

    let program = Program {
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
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: dfa_prefixes,
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes,
        match_type: MatchType::Dfa,
    };

    exec_read_only.should_suffix_scan();
}

#[test]
fn test_should_suffix_scan_case_2() {
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![b"xyz".to_vec()]));
    let lcp = FreqyPacked::new(vec![b"b".to_vec()]);
    let dfa_prefixes = LiteralSearcher::prefixes(Literals::new(vec![b"xy".to_vec()]));

    let program = Program {
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
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: dfa_prefixes,
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes,
        match_type: MatchType::Dfa,
    };

    exec_read_only.should_suffix_scan();
}

#[test]
fn test_should_suffix_scan_case_3() {
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![b"ghi".to_vec()]));
    let lcp = FreqyPacked::new(vec![b"c".to_vec()]);
    let dfa_prefixes = LiteralSearcher::prefixes(Literals::new(vec![b"gh".to_vec()]));

    let program = Program {
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
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: dfa_prefixes,
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["ghi".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes,
        match_type: MatchType::Dfa,
    };

    exec_read_only.should_suffix_scan();
}

