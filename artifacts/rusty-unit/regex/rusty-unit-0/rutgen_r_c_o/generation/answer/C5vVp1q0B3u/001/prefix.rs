// Answer 0

#[test]
fn test_capture_name_idx_with_non_empty_map() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;

    struct ExecReadOnly {
        nfa: NfaData,
    }

    struct NfaData {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    let mut map = HashMap::new();
    map.insert("group1".to_string(), 0);
    map.insert("group2".to_string(), 1);

    let data = ExecReadOnly {
        nfa: NfaData { 
            capture_name_idx: Arc::new(map),
        },
    };

    let exec = ExecNoSync { ro: &Arc::new(data), cache: &RefCell::new(ProgramCacheInner{}) };
    let exec_str = ExecNoSyncStr(exec);
    let _result = exec_str.capture_name_idx();
}

#[test]
fn test_capture_name_idx_with_empty_map() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;

    struct ExecReadOnly {
        nfa: NfaData,
    }

    struct NfaData {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    let map = HashMap::new();

    let data = ExecReadOnly {
        nfa: NfaData { 
            capture_name_idx: Arc::new(map),
        },
    };

    let exec = ExecNoSync { ro: &Arc::new(data), cache: &RefCell::new(ProgramCacheInner{}) };
    let exec_str = ExecNoSyncStr(exec);
    let _result = exec_str.capture_name_idx();
}

#[test]
fn test_capture_name_idx_with_large_map() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;

    struct ExecReadOnly {
        nfa: NfaData,
    }

    struct NfaData {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert(format!("group{}", i), i);
    }

    let data = ExecReadOnly {
        nfa: NfaData { 
            capture_name_idx: Arc::new(map),
        },
    };

    let exec = ExecNoSync { ro: &Arc::new(data), cache: &RefCell::new(ProgramCacheInner{}) };
    let exec_str = ExecNoSyncStr(exec);
    let _result = exec_str.capture_name_idx();
}

#[test]
#[should_panic]
fn test_capture_name_idx_with_panic_condition() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;

    struct ExecReadOnly {
        nfa: NfaData,
    }

    struct NfaData {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    let mut map = HashMap::new();
    map.insert("group1".to_string(), usize::MAX); // This should trigger a panic in some scenario

    let data = ExecReadOnly {
        nfa: NfaData { 
            capture_name_idx: Arc::new(map),
        },
    };

    let exec = ExecNoSync { ro: &Arc::new(data), cache: &RefCell::new(ProgramCacheInner{}) };
    let exec_str = ExecNoSyncStr(exec);
    let _result = exec_str.capture_name_idx();
}

