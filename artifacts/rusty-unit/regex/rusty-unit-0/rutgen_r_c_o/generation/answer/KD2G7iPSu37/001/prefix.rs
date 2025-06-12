// Answer 0

#[test]
fn test_is_match_at_nothing_type() {
    let text: &[u8] = b"Sample text that does not match the regex";
    let start: usize = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec!["Sample regex".to_string()],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::new(),
            dfa_size_limit: 0,
        },
        dfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::new(),
            dfa_size_limit: 0,
        },
        dfa_reverse: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::new(),
            dfa_size_limit: 0,
        },
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::Nothing,
    });

    let cache = RefCell::new(ProgramCacheInner{
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let _ = exec.is_match_at(text, start);
}

