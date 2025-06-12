// Answer 0

#[test]
fn test_capture_name_idx_with_empty_mapping() {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    struct MockProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }
    
    struct MockExecReadOnly {
        nfa: MockProgram,
    }
    
    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }
    
    let capture_name_idx = Arc::new(HashMap::new());
    let mock_program = MockProgram { capture_name_idx };
    let mock_read_only = MockExecReadOnly { nfa: mock_program };
    let mock_exec = MockExec { ro: Arc::new(mock_read_only) };

    let result = mock_exec.capture_name_idx();
    assert!(result.is_empty());
}

#[test]
fn test_capture_name_idx_with_single_mapping() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct MockExecReadOnly {
        nfa: MockProgram,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    let mut map = HashMap::new();
    map.insert("group1".to_string(), 0);
    let capture_name_idx = Arc::new(map);
    let mock_program = MockProgram { capture_name_idx };
    let mock_read_only = MockExecReadOnly { nfa: mock_program };
    let mock_exec = MockExec { ro: Arc::new(mock_read_only) };

    let result = mock_exec.capture_name_idx();
    assert_eq!(result.len(), 1);
    assert_eq!(result.get("group1"), Some(&0));
}

#[test]
fn test_capture_name_idx_with_multiple_mappings() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct MockExecReadOnly {
        nfa: MockProgram,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    let mut map = HashMap::new();
    map.insert("group1".to_string(), 0);
    map.insert("group2".to_string(), 1);
    let capture_name_idx = Arc::new(map);
    let mock_program = MockProgram { capture_name_idx };
    let mock_read_only = MockExecReadOnly { nfa: mock_program };
    let mock_exec = MockExec { ro: Arc::new(mock_read_only) };

    let result = mock_exec.capture_name_idx();
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("group1"), Some(&0));
    assert_eq!(result.get("group2"), Some(&1));
}

