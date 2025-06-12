// Answer 0

#[test]
fn test_forward_many_single_match() {
    use std::collections::HashMap;

    // Setup a simple Program with a regex that should match "a"
    let insts = vec![
        Inst::Char(InstChar { c: b'a' }),
        Inst::Match(0),
    ];
    let matches = vec![0];
    let prog = Program {
        insts,
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![State { data: vec![0; 1].into_boxed_slice() }],
        start_states: vec![1],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache {
            inner: cache_inner,
            qcur,
            qnext,
        },
        dfa_reverse: dfa::Cache::default(),
    };

    let text = b"a";
    let at = 0;
    let mut matches_out = vec![false];

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches_out, text, at);

    assert!(matches_out[0]);
    assert!(result.is_match());
}

#[test]
fn test_forward_many_multiple_matches() {
    use std::collections::HashMap;

    // Setup a Program that can match "ab"
    let insts = vec![
        Inst::Char(InstChar { c: b'a' }),
        Inst::Char(InstChar { c: b'b' }),
        Inst::Match(0),
        Inst::Character(InstChar { c: b'c' }),
        Inst::Match(1),
    ];
    let matches = vec![0, 1];
    let prog = Program {
        insts,
        matches,
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![State { data: vec![0; 1].into_boxed_slice() }; 2],
        start_states: vec![1],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache {
            inner: cache_inner,
            qcur,
            qnext,
        },
        dfa_reverse: dfa::Cache::default(),
    };

    let text = b"abc";
    let at = 0;
    let mut matches_out = vec![false; prog.matches.len()];

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches_out, text, at);

    assert!(matches_out[0]);
    assert!(matches_out[1]);
    assert!(result.is_match());
}

#[test]
fn test_forward_many_no_match_empty_text() {
    use std::collections::HashMap;

    // Setup a Program that can match "a"
    let insts = vec![
        Inst::Char(InstChar { c: b'a' }),
        Inst::Match(0),
    ];
    let matches = vec![0];
    let prog = Program {
        insts,
        matches,
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![],
        states: vec![State { data: vec![0; 1].into_boxed_slice() }],
        start_states: vec![1],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache {
            inner: cache_inner,
            qcur,
            qnext,
        },
        dfa_reverse: dfa::Cache::default(),
    };

    let text: &[u8] = b"";
    let at = 0;
    let mut matches_out = vec![false];

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches_out, text, at);

    assert!(!matches_out[0]);
    assert!(!result.is_match());
}

