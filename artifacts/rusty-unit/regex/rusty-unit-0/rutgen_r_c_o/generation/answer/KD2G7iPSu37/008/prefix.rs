// Answer 0

#[test]
fn test_is_match_at_dfa_suffix_match() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("abc")],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::new(), backtrack: backtrack::Cache::new(), dfa: dfa::Cache::new(), dfa_reverse: dfa::Cache::new() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"abc";
    let start = 0;

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_dfa_suffix_match_long_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("xyz")],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: true, is_dfa: false, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: true, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: true, is_dfa: true, is_reverse: true, is_anchored_start: true, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::new(), backtrack: backtrack::Cache::new(), dfa: dfa::Cache::new(), dfa_reverse: dfa::Cache::new() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"xyzxyzxyzxyzxyzxyz";
    let start = 0;

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_dfa_suffix_mid_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 10 },
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::new(), backtrack: backtrack::Cache::new(), dfa: dfa::Cache::new(), dfa_reverse: dfa::Cache::new() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"this is a test string";
    let start = 10;

    exec.is_match_at(text, start);
}

