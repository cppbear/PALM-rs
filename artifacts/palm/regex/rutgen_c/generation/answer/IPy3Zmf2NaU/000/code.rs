// Answer 0

#[test]
fn test_cache_new_with_empty_program() {
    use std::sync::Arc;
    
    let insts: Vec<Inst> = vec![];
    let matches: Vec<InstPtr> = vec![];
    let captures: Vec<Option<String>> = vec![];
    let capture_name_idx = Arc::new(HashMap::new());
    let byte_classes = vec![0; 256];
    
    let prog = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes,
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    
    let cache = Cache::new(&prog);
    
    assert_eq!(cache.inner.states.len(), 0);
    assert_eq!(cache.inner.start_states.len(), 256);
    assert_eq!(cache.qcur.len(), 0);
    assert_eq!(cache.qnext.len(), 0);
}

#[test]
fn test_cache_new_with_non_empty_program() {
    use std::sync::Arc;
    
    let insts: Vec<Inst> = vec![Inst::Match(0)];
    let matches: Vec<InstPtr> = vec![0];
    let captures: Vec<Option<String>> = vec![Some("group1".to_string())];
    let capture_name_idx = Arc::new(HashMap::new());
    let byte_classes = vec![0; 256];
    
    let prog = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
        start: 0,
        byte_classes,
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };
    
    let cache = Cache::new(&prog);
    
    assert_eq!(cache.inner.states.len(), 0);
    assert_eq!(cache.inner.start_states.len(), 256);
    assert_eq!(cache.qcur.len(), 0);
    assert_eq!(cache.qnext.len(), 0);
}

