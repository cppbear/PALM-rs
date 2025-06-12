// Answer 0

#[test]
fn test_cache_new() {
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let cache = Cache::new(&program);

    assert_eq!(cache.clist.slots_per_thread, 0);
    assert_eq!(cache.nlist.slots_per_thread, 0);
    assert!(cache.stack.is_empty());
}

#[test]
fn test_threads_new() {
    let threads = Threads::new();

    assert_eq!(threads.slots_per_thread, 0);
    assert!(threads.caps.is_empty());
}

