// Answer 0

#[test]
fn test_many_matches_at_basic_match() {
    let text = b"example";
    let mut matches = vec![false];
    let ro = Arc::new(ExecReadOnly {
        res: vec!["^ex".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::default(), backtrack: backtrack::Cache::default(), dfa: dfa::Cache::default(), dfa_reverse: dfa::Cache::default() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.many_matches_at(&mut matches, text, 0);
}

#[test]
fn test_many_matches_at_multiple_matches() {
    let text = b"regex example with multiple matches regex";
    let mut matches = vec![false; 2];
    let ro = Arc::new(ExecReadOnly {
        res: vec!["regex".to_string(), "example".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::default(), backtrack: backtrack::Cache::default(), dfa: dfa::Cache::default(), dfa_reverse: dfa::Cache::default() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.many_matches_at(&mut matches, text, 0);
}

#[test]
fn test_many_matches_at_empty_text() {
    let text = b"";
    let mut matches = vec![false];
    let ro = Arc::new(ExecReadOnly {
        res: vec!["^ex".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: true, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::default(), backtrack: backtrack::Cache::default(), dfa: dfa::Cache::default(), dfa_reverse: dfa::Cache::default() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.many_matches_at(&mut matches, text, 0);
}

#[test]
fn test_many_matches_at_long_text() {
    let text = b"this is a long text that is meant to test performance and matching many patterns successfully with regex that are considered complex patterns";
    let mut matches = vec![false; 3];
    let ro = Arc::new(ExecReadOnly {
        res: vec!["complex".to_string(), "patterns".to_string(), "successful".to_string()],
        nfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        dfa_reverse: Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: true, is_bytes: false, is_dfa: true, is_reverse: true, is_anchored_start: false, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::default(), dfa_size_limit: 1024 },
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::DfaMany,
    });
    let cache = RefCell::new(ProgramCacheInner { pikevm: pikevm::Cache::default(), backtrack: backtrack::Cache::default(), dfa: dfa::Cache::default(), dfa_reverse: dfa::Cache::default() });
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let result = exec.many_matches_at(&mut matches, text, 0);
}

