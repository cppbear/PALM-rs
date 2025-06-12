// Answer 0

#[test]
fn test_capture_name_idx_empty() {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DummyProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }
    
    let capture_name_idx = Arc::new(HashMap::new());
    let program = DummyProgram { capture_name_idx: capture_name_idx.clone() };
    
    struct DummyExecReadOnly {
        nfa: DummyProgram,
    }

    let exec_read_only = Arc::new(DummyExecReadOnly { nfa: program });
    let exec = Exec {
        ro: exec_read_only,
        cache: CachedThreadLocal::new(),
    };
    
    let _ = exec.capture_name_idx();
}

#[test]
fn test_capture_name_idx_single_group() {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DummyProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }
    
    let mut capture_map = HashMap::new();
    capture_map.insert("group1".to_string(), 0);
    let capture_name_idx = Arc::new(capture_map);
    let program = DummyProgram { capture_name_idx: capture_name_idx.clone() };
    
    struct DummyExecReadOnly {
        nfa: DummyProgram,
    }

    let exec_read_only = Arc::new(DummyExecReadOnly { nfa: program });
    let exec = Exec {
        ro: exec_read_only,
        cache: CachedThreadLocal::new(),
    };
    
    let _ = exec.capture_name_idx();
}

#[test]
fn test_capture_name_idx_multiple_groups() {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DummyProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }
    
    let mut capture_map = HashMap::new();
    for i in 0..20 {
        let group_name = format!("group{}", i);
        capture_map.insert(group_name, i);
    }
    let capture_name_idx = Arc::new(capture_map);
    let program = DummyProgram { capture_name_idx: capture_name_idx.clone() };
    
    struct DummyExecReadOnly {
        nfa: DummyProgram,
    }

    let exec_read_only = Arc::new(DummyExecReadOnly { nfa: program });
    let exec = Exec {
        ro: exec_read_only,
        cache: CachedThreadLocal::new(),
    };
    
    let _ = exec.capture_name_idx();
}

#[test]
fn test_capture_name_idx_boundary_group_names() {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::sync::Arc;

    struct DummyProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }
    
    let mut capture_map = HashMap::new();
    let short_name = "g".to_string();
    let long_name = "group_that_is_longer_than_fifty_characters_which_should_not_be_possible".to_string();
    
    capture_map.insert(short_name, 0);
    capture_map.insert(long_name.clone(), 1); // This should not block, but is an edge case
    let capture_name_idx = Arc::new(capture_map);
    let program = DummyProgram { capture_name_idx: capture_name_idx.clone() };
    
    struct DummyExecReadOnly {
        nfa: DummyProgram,
    }

    let exec_read_only = Arc::new(DummyExecReadOnly { nfa: program });
    let exec = Exec {
        ro: exec_read_only,
        cache: CachedThreadLocal::new(),
    };
    
    let _ = exec.capture_name_idx();
}

