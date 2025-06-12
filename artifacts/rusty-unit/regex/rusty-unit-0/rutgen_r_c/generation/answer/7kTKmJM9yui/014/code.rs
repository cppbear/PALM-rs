// Answer 0

#[test]
fn test_many_matches_at_dfa_match() {
    use std::cell::RefCell;

    struct MockExecReadOnly {
        match_type: MatchType,
        dfa: Program,
        is_anchored_end: bool,
    }

    struct MockCache {
        inner: ProgramCacheInner,
    }

    let dfa_program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let ro = Arc::new(MockExecReadOnly {
        match_type: MatchType::Dfa,
        dfa: dfa_program,
        is_anchored_end: true,
    });

    let cache = RefCell::new(MockCache {
        inner: ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        },
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    let mut matches = vec![false];
    let text = b"some sample text that matches";
    let start = 0;

    let result = exec.many_matches_at(&mut matches, text, start);

    assert!(result);
    assert!(matches[0], "Expected first match to be true");
}

