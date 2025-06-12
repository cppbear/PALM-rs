// Answer 0

#[test]
fn test_step_with_non_empty_check_and_valid_range() {
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 10, end: 20 })],
        matches: vec![InstPtr::from(0)],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::from(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        inner: CacheInner::new(),
        qcur: SparseSet::new(),
        qnext: SparseSet::new(),
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    let input = MockInput::new(vec![15]);
    let start = 0;
    let mut bounded = Bounded {
        prog: &program,
        input,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt { pos: 0, c: Char(15), byte: Some(15), len: 1 };
    let ip = 0;
    
    bounded.step(ip, at);
}

#[test]
fn test_step_with_visited_set_and_invalid_byte() {
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 10, end: 20 })],
        matches: vec![InstPtr::from(0)],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::from(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        inner: CacheInner::new(),
        qcur: SparseSet::new(),
        qnext: SparseSet::new(),
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    let input = MockInput::new(vec![5]);
    let start = 0;
    let mut bounded = Bounded {
        prog: &program,
        input,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt { pos: 0, c: Char(5), byte: Some(5), len: 1 };
    let ip = 0;
    
    bounded.step(ip, at);
}

#[test]
fn test_step_with_has_visited_set_true() {
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 5, end: 10 })],
        matches: vec![InstPtr::from(0)],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::from(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        inner: CacheInner::new(),
        qcur: SparseSet::new(),
        qnext: SparseSet::new(),
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    let input = MockInput::new(vec![7]);
    let start = 0;
    let mut bounded = Bounded {
        prog: &program,
        input,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt { pos: 0, c: Char(7), byte: Some(7), len: 1 };
    let ip = 0;
    
    bounded.step(ip, at);
}

#[test]
fn test_step_multiple_byte_matches() {
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 0, end: 255 })],
        matches: vec![InstPtr::from(1)],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::from(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        inner: CacheInner::new(),
        qcur: SparseSet::new(),
        qnext: SparseSet::new(),
    };

    let mut matches = vec![false; 256];
    let mut slots = vec![None; 256];
    let input = MockInput::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let start = 0;
    let mut bounded = Bounded {
        prog: &program,
        input,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt { pos: 0, c: Char(15), byte: Some(15), len: 1 };
    let ip = 0;

    bounded.step(ip, at);
}

