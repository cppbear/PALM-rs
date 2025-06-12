// Answer 0

#[test]
fn test_forward_many_match() {
    use std::sync::Arc;

    let prog = Program {
        insts: vec![Inst::Match(0)], // One match instruction.
        matches: vec![0], // Corresponding to the match instruction.
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()), // Empty for this test.
        start: 0,
        byte_classes: vec![0; 256], // Assume 256 byte classes.
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![State { data: Box::new([]) }],
        start_states: vec![0, STATE_DEAD, STATE_UNKNOWN],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache {
            inner: cache_inner,
            qcur: SparseSet::default(),
            qnext: SparseSet::default(),
        },
        dfa_reverse: dfa::Cache::default(),
    };

    let text = b"abc"; // Input text to match against.
    let at = 0;
    let mut matches = vec![false]; // Matches has to be length 1.

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, at);

    assert!(matches[0]);
    match result {
        Result::Match(_) => {}
        _ => panic!("Expected a match result"),
    }
}

#[test]
fn test_forward_many_no_match() {
    use std::sync::Arc;

    let prog = Program {
        insts: vec![Inst::Char(InstChar { ch: 'z' })], // Match only 'z'.
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![State { data: Box::new([]) }],
        start_states: vec![0, STATE_DEAD, STATE_UNKNOWN],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache {
            inner: cache_inner,
            qcur: SparseSet::default(),
            qnext: SparseSet::default(),
        },
        dfa_reverse: dfa::Cache::default(),
    };

    let text = b"abc"; // Input text does not match.
    let at = 0;
    let mut matches = vec![false]; // Matches has to be length 1.

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, at);

    assert!(!matches[0]); // Should remain false.
    match result {
        Result::NoMatch(_) => {}
        _ => panic!("Expected no match result"),
    }
}

