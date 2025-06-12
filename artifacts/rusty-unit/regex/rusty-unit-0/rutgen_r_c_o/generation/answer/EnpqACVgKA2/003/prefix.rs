// Answer 0

#[test]
fn test_should_suffix_scan_lcs_len_0_lcp_len_0() {
    let suffixes = LiteralSearcher::empty();
    let dfa_prefixes = FreqyPacked::new(vec![]);
    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: suffixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Empty),
    };
    exec_read_only.should_suffix_scan();
}

#[test]
fn test_should_suffix_scan_lcs_len_1_lcp_len_0() {
    let suffixes = LiteralSearcher::suffixes(Literals::from(vec![b'a']));
    let dfa_prefixes = FreqyPacked::new(vec![]);
    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: suffixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Empty),
    };
    exec_read_only.should_suffix_scan();
}

#[test]
fn test_should_suffix_scan_lcs_len_2_lcp_len_0() {
    let suffixes = LiteralSearcher::suffixes(Literals::from(vec![b'a', b'b']));
    let dfa_prefixes = FreqyPacked::new(vec![]);
    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: suffixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Empty),
    };
    exec_read_only.should_suffix_scan();
}

#[test]
fn test_should_suffix_scan_lcs_len_2_lcp_len_1() {
    let suffixes = LiteralSearcher::suffixes(Literals::from(vec![b'a', b'b']));
    let dfa_prefixes = FreqyPacked::new(vec![b'x']);
    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: suffixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Empty),
    };
    exec_read_only.should_suffix_scan();
}

#[test]
fn test_should_suffix_scan_lcs_len_2_lcp_len_2() {
    let suffixes = LiteralSearcher::suffixes(Literals::from(vec![b'a', b'b']));
    let dfa_prefixes = FreqyPacked::new(vec![b'x', b'y']);
    let exec_read_only = ExecReadOnly {
        res: vec![],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: suffixes.clone(), dfa_size_limit: 0 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: InstPtr { index: 0 }, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: dfa_prefixes.clone(), dfa_size_limit: 0 },
        suffixes,
        match_type: MatchType::Nfa(MatchNfaType::Empty),
    };
    exec_read_only.should_suffix_scan();
}

