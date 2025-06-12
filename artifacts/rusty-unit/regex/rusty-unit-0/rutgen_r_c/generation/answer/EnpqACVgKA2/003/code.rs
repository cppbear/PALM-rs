// Answer 0

#[test]
fn test_should_suffix_scan_with_suffixes_empty() {
    let dfa_prefixes = FreqyPacked::new(vec![1, 2]); // lcp length = 2
    let lcs = FreqyPacked::new(vec![3]); // lcs length = 1 < 3
    let suffixes = LiteralSearcher::suffixes(Literals::empty()); // empty suffixes

    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Dfa,
    };

    assert_eq!(exec_read_only.should_suffix_scan(), false);
}

#[test]
fn test_should_suffix_scan_with_lcs_len_below_threshold() {
    let dfa_prefixes = FreqyPacked::new(vec![1, 2]); // lcp length = 2
    let lcs = FreqyPacked::new(vec![1, 2]); // lcs length = 2 < 3
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![1, 2, 3])); // not empty

    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Dfa,
    };

    assert_eq!(exec_read_only.should_suffix_scan(), false);
}

#[test]
fn test_should_suffix_scan_valid_params() {
    let dfa_prefixes = FreqyPacked::new(vec![1, 2]); // lcp length = 2
    let lcs = FreqyPacked::new(vec![3, 4, 5]); // lcs length = 3
    let suffixes = LiteralSearcher::suffixes(Literals::new(vec![1, 2, 3])); // not empty

    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Dfa,
    };

    assert_eq!(exec_read_only.should_suffix_scan(), true);
}

