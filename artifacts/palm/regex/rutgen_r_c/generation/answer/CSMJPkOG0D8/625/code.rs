// Answer 0

#[test]
fn test_exec_at_no_match() {
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let cache = ProgramCache::new();
    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: program.insts.len(),
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };
    
    let text: &[u8] = b"";
    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
    assert!(matches!(result, Result::NoMatch(_)));
}

#[test]
fn test_exec_at_match() {
    let program = Program {
        insts: vec![/* construct instructions that allow matches */],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let cache = ProgramCache::new();
    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let text: &[u8] = b"abc"; // input text that is expected to match
    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
    assert!(matches!(result, Result::Match(_)));
}

#[test]
#[should_panic]
fn test_exec_at_panic_conditions() {
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let cache = ProgramCache::new();
    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let text: &[u8] = b"";
    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
    // causing panic due to self.next_state not being valid leading to panic conditions
    let _ = fsm.next_state(&mut qcur, &mut qnext, STATE_UNKNOWN, Byte::eof());
}

