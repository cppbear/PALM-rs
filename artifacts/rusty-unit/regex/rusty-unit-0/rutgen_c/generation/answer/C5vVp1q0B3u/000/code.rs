// Answer 0

#[test]
fn test_capture_name_idx() {
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::cell::RefCell;

    struct ExecReadOnly {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct ExecNoSync<'c> {
        ro: &'c Arc<ExecReadOnly>,
        cache: &'c RefCell<ProgramCacheInner>,
    }

    struct ExecNoSyncStr<'c>(ExecNoSync<'c>);

    let capture_map: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let exec_read_only = ExecReadOnly {
        capture_name_idx: capture_map.clone(),
    };
    let cache = RefCell::new(ProgramCacheInner {});
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };
    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);

    let result = exec_no_sync_str.capture_name_idx();
    assert_eq!(Arc::strong_count(&result), 2);
}

#[test]
fn test_capture_name_idx_empty_map() {
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::cell::RefCell;

    struct ExecReadOnly {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct ExecNoSync<'c> {
        ro: &'c Arc<ExecReadOnly>,
        cache: &'c RefCell<ProgramCacheInner>,
    }

    struct ExecNoSyncStr<'c>(ExecNoSync<'c>);

    let capture_map: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let exec_read_only = ExecReadOnly {
        capture_name_idx: capture_map.clone(),
    };
    let cache = RefCell::new(ProgramCacheInner {});
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(exec_read_only),
        cache: &cache,
    };
    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);

    let result = exec_no_sync_str.capture_name_idx();
    assert!(result.is_empty());
}

