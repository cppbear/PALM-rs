// Answer 0

#[test]
fn test_should_suffix_scan_empty_suffixes() {
    let suffixes = LiteralSearcher::empty();
    let prefixes = FreqyPacked::empty();
    let dfa = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: suffixes.clone(),
        dfa_size_limit: 0,
    };
    
    let exec_read_only = ExecReadOnly {
        res: Vec::new(),
        nfa: dfa.clone(),
        dfa: dfa.clone(),
        dfa_reverse: dfa.clone(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    assert_eq!(exec_read_only.should_suffix_scan(), false);
}

#[test]
fn test_should_suffix_scan_meaty_suffix() {
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![b"suffix1".to_vec(), b"suffix2".to_vec()]));
    let prefixes = FreqyPacked::new(vec![0u8; 1]);
    let dfa = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes,
        dfa_size_limit: 0,
    };
    
    let exec_read_only = ExecReadOnly {
        res: Vec::new(),
        nfa: dfa.clone(),
        dfa: dfa.clone(),
        dfa_reverse: dfa.clone(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    assert_eq!(exec_read_only.should_suffix_scan(), true);
}

#[test]
fn test_should_suffix_scan_short_suffix_not_meaty() {
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![b"su".to_vec()])); // less than 3 characters
    let prefixes = FreqyPacked::new(vec![0u8; 1]);
    let dfa = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes,
        dfa_size_limit: 0,
    };
    
    let exec_read_only = ExecReadOnly {
        res: Vec::new(),
        nfa: dfa.clone(),
        dfa: dfa.clone(),
        dfa_reverse: dfa.clone(),
        suffixes,
        match_type: MatchType::Nothing,
    };

    assert_eq!(exec_read_only.should_suffix_scan(), false);
}

