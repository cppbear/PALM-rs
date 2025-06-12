// Answer 0

#[test]
fn test_capture_name_idx_empty() {
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::cell::RefCell;

    struct ExecReadOnly {
        nfa: Arc<HashMap<String, usize>>,
    }

    struct ExecNoSync<'c> {
        ro: &'c Arc<ExecReadOnly>,
        cache: &'c RefCell<ProgramCacheInner>,
    }

    struct ExecNoSyncStr<'c>(ExecNoSync<'c>);

    let empty_map: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    let read_only = Arc::new(ExecReadOnly { nfa: empty_map.clone() });
    let cache: RefCell<ProgramCacheInner> = RefCell::new(ProgramCacheInner {});
    let exec_no_sync = ExecNoSync { ro: &read_only, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);

    assert_eq!(exec_no_sync_str.capture_name_idx(), &empty_map);
}

#[test]
fn test_capture_name_idx_multiple_entries() {
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::cell::RefCell;

    struct ExecReadOnly {
        nfa: Arc<HashMap<String, usize>>,
    }

    struct ExecNoSync<'c> {
        ro: &'c Arc<ExecReadOnly>,
        cache: &'c RefCell<ProgramCacheInner>,
    }

    struct ExecNoSyncStr<'c>(ExecNoSync<'c>);

    let mut map = HashMap::new();
    map.insert("first_capture".to_string(), 0);
    map.insert("second_capture".to_string(), 1);
    let capture_map: Arc<HashMap<String, usize>> = Arc::new(map);
    
    let read_only = Arc::new(ExecReadOnly { nfa: capture_map.clone() });
    let cache: RefCell<ProgramCacheInner> = RefCell::new(ProgramCacheInner {});
    let exec_no_sync = ExecNoSync { ro: &read_only, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);

    assert_eq!(exec_no_sync_str.capture_name_idx(), &capture_map);
}

#[test]
#[should_panic] // In the context of the real use, if self.0 is not properly initialized, this would panic
fn test_capture_name_idx_panics_when_not_initialized() {
    #[derive(Debug)]
    struct NotInitialized;

    struct ExecNoSync<'c> {
        ro: &'c NotInitialized,
        cache: &'c RefCell<ProgramCacheInner>,
    }

    struct ExecNoSyncStr<'c>(ExecNoSync<'c>);

    let not_initialized = NotInitialized;
    let cache: RefCell<ProgramCacheInner> = RefCell::new(ProgramCacheInner {});
    let exec_no_sync = ExecNoSync { ro: &not_initialized, cache: &cache };
    let exec_no_sync_str = ExecNoSyncStr(exec_no_sync);
    
    // This should panic as no valid capture_name_idx exists
    let _ = exec_no_sync_str.capture_name_idx();
}

