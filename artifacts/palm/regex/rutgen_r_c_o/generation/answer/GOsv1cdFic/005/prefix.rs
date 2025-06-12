// Answer 0

#[test]
fn test_forward_many_case1() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![false];
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
        dfa_size_limit: 10,
    };
    let mut cache_inner = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: dfa::Cache::default()
    };
    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
        qnext: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
    };
    
    let text = b"test";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches[..], text, at);
}

#[test]
fn test_forward_many_case2() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![false];
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
        dfa_size_limit: 10,
    };
    let mut cache_inner = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: dfa::Cache::default()
    };
    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
        qnext: SparseSet {
            dense: vec![],
            sparse: vec![],
            size: 0,
        },
    };
    
    let text = b"match";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches[..], text, at);
}

