// Answer 0

#[test]
fn test_exec_at_reverse_case_1() {
    let mut cache = CacheInner { compiled: HashMap::new(), trans: Transitions::new(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 };
    let prog = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 256 };
    let mut fsm = Fsm { prog: &prog, start: 0, at: 0, quit_after_match: false, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    let mut qcur = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let mut qnext = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let text = vec![b'a'; 5];
    let _result = unsafe { fsm.exec_at_reverse(&mut qcur, &mut qnext, &text) };
}

#[test]
fn test_exec_at_reverse_case_2() {
    let mut cache = CacheInner { compiled: HashMap::new(), trans: Transitions::new(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 };
    let prog = Program { insts: vec![], matches: vec![0], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![0], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 256 };
    let mut fsm = Fsm { prog: &prog, start: 0, at: 4, quit_after_match: true, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    let mut qcur = SparseSet { dense: vec![0], sparse: vec![0], size: 1 };
    let mut qnext = SparseSet { dense: vec![0], sparse: vec![0], size: 1 };
    let text = vec![b'b'; 2];
    let _result = unsafe { fsm.exec_at_reverse(&mut qcur, &mut qnext, &text) };
}

#[test]
fn test_exec_at_reverse_case_3() {
    let mut cache = CacheInner { compiled: HashMap::new(), trans: Transitions::new(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 };
    let prog = Program { insts: vec![], matches: vec![0], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![1], only_utf8: false, is_bytes: true, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 256 };
    let mut fsm = Fsm { prog: &prog, start: 0, at: 3, quit_after_match: false, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    let mut qcur = SparseSet { dense: vec![0], sparse: vec![0], size: 1 };
    let mut qnext = SparseSet { dense: vec![0], sparse: vec![0], size: 1 };
    let text = vec![b'c'; 1];
    let _result = unsafe { fsm.exec_at_reverse(&mut qcur, &mut qnext, &text) };
}

#[test]
#[should_panic]
fn test_exec_at_reverse_case_4() {
    let mut cache = CacheInner { compiled: HashMap::new(), trans: Transitions::new(), states: vec![], start_states: vec![], stack: vec![], flush_count: 0, size: 0 };
    let prog = Program { insts: vec![], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 256 };
    let mut fsm = Fsm { prog: &prog, start: 0, at: 0, quit_after_match: false, last_match_si: STATE_UNKNOWN, last_cache_flush: 0, cache: &mut cache };
    let mut qcur = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let mut qnext = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let text = vec![];
    let _result = unsafe { fsm.exec_at_reverse(&mut qcur, &mut qnext, &text) }; 
}

