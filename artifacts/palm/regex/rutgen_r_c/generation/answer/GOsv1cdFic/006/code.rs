// Answer 0

#[test]
fn test_forward_many_single_match() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'a', b'b'],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([0]) }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet::default(),
        qnext: SparseSet::default(),
    };

    let text = b"abc";
    let mut matches = vec![false];

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
    
    assert!(result.is_match());
    assert!(matches[0]);
}

#[test]
fn test_forward_many_no_match() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'a', b'b'],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([0]) }],
        start_states: vec![STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet::default(),
        qnext: SparseSet::default(),
    };

    let text = b"xyz";
    let mut matches = vec![false];

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
    
    assert!(!result.is_match());
    assert!(!matches[0]);
}

#[test]
fn test_forward_many_multiple_matches() {
    let prog = Program {
        insts: vec![
            Inst::Char(InstChar { c: b'a' }),
            Inst::Match(0)
        ],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![b'a', b'b'],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([0]) }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet::default(),
        qnext: SparseSet::default(),
    };

    let text = b"abcabc";
    let mut matches = vec![false, false];

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
    
    assert!(result.is_match());
    assert!(matches[0]);
    assert!(!matches[1]);
}

