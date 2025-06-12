// Answer 0

#[test]
fn test_forward_many_no_match() {
    use std::sync::Arc;

    // Creating a dummy Program with no actual matches.
    let program = Program {
        insts: vec![Inst::Match(0)], // Only one instruction which is a match
        matches: vec![0], // It matches on index 0
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    // Initialize a matching cache.
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([]) }],
        start_states: vec![STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut program_cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
    };

    let mut matches = vec![false]; // The match array
    let text = b"no match here"; // Input text
    let at = 0; // Starting position

    // Run the function under test
    let result = Fsm::forward_many(&program, &program_cache, &mut matches, text, at);

    // Assert the expected output
    assert!(matches.len() == program.matches.len());
    assert!(matches[0] == false); // Should not have matched
    match result {
        Result::NoMatch(pos) => assert_eq!(pos, at), // Expect NoMatch with the starting position
        _ => panic!("Expected NoMatch, got {:?}", result),
    }
}

#[test]
fn test_forward_many_some_state_dead() {
    use std::sync::Arc;

    // Creating a dummy Program with a forced state dead condition
    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    // Initialize a matching cache
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([]) }],
        start_states: vec![STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut program_cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
    };

    let mut matches = vec![false]; // The match array
    let text = b"any text"; // Input text
    let at = 0; // Starting position

    // Run the function under test
    let result = Fsm::forward_many(&program, &program_cache, &mut matches, text, at);

    // Assert the expected output
    assert!(matches.len() == program.matches.len());
    assert!(matches[0] == false); // Should not have matched
    match result {
        Result::NoMatch(pos) => assert_eq!(pos, at), // Expect NoMatch with the starting position
        _ => panic!("Expected NoMatch, got {:?}", result),
    }
}

#[test]
fn test_forward_many_with_state_start() {
    use std::sync::Arc;

    // Create a program where start is directly connected to a Match state.
    let program = Program {
        insts: vec![Inst::Char(InstChar { c: b'a' }), Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([]) }],
        start_states: vec![STATE_UNKNOWN],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut program_cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
    };

    let mut matches = vec![false]; // The match array
    let text = b"a"; // Input text
    let at = 0; // Starting position

    // Run the function under test
    let result = Fsm::forward_many(&program, &program_cache, &mut matches, text, at);

    // Assert the expected output
    assert!(matches.len() == program.matches.len());
    assert!(matches[0]); // Should have matched
    match result {
        Result::Match(pos) => assert_eq!(pos, 1), // Match at position 1 (end)
        _ => panic!("Expected Match, got {:?}", result),
    }
}

