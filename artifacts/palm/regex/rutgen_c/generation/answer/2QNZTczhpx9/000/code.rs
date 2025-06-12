// Answer 0

#[test]
fn test_find_at_valid_match() {
    struct ExecReadOnly {};
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec);

    let text = "sample text for matching";
    let start = 0;
    let result = exec_no_sync_str.find_at(text, start);
    assert!(result.is_some());
}

#[test]
fn test_find_at_no_match() {
    struct ExecReadOnly {};
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec);

    let text = "no match here";
    let start = 0;
    let result = exec_no_sync_str.find_at(text, start);
    assert_eq!(result, None);
}

#[test]
fn test_find_at_start_index_out_of_bounds() {
    struct ExecReadOnly {};
    let ro = Arc::new(ExecReadOnly {});
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec);

    let text = "short text";
    let start = text.len();  // start index out of bounds
    let result = exec_no_sync_str.find_at(text, start);
    assert_eq!(result, None);
}

