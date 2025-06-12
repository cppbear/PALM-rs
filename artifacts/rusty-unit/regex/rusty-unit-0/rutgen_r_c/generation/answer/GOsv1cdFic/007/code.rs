// Answer 0

#[test]
fn test_forward_many_empty_input() {
    let prog = Program {
        insts: vec![],
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
        states: vec![],
        start_states: vec![STATE_DEAD; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false];
    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, b"".as_ref(), 0);

    assert!(matches.len() == prog.matches.len());
    assert!(result.is_match() == false);
}

#[test]
fn test_forward_many_single_match() {
    let prog = Program {
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

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![1; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false];
    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, b"a".as_ref(), 0);
    
    assert!(matches.len() == prog.matches.len());
    assert!(matches[0] == true);
    assert!(result.is_match() == true);
}

#[test]
fn test_forward_many_no_match() {
    let prog = Program {
        insts: vec![Inst::Match(1)],
        matches: vec![1],
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
        states: vec![],
        start_states: vec![STATE_DEAD; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false];
    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, b"b".as_ref(), 0);
    
    assert!(matches.len() == prog.matches.len());
    assert!(matches[1] == false);
    assert!(result.is_match() == false);
}

#[test]
fn test_forward_many_with_multiple_matches() {
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
        matches: vec![0, 1],
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
        states: vec![],
        start_states: vec![1; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache { inner: cache_inner, qcur: SparseSet::default(), qnext: SparseSet::default() },
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false; 2];
    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, b"aa".as_ref(), 0);

    assert!(matches.len() == prog.matches.len());
    assert!(matches[0] == true);
    assert!(matches[1] == true);
    assert!(result.is_match() == true);
}

