// Answer 0

#[test]
fn test_capture_name_idx_empty() {
    struct DummyProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct DummyExecReadOnly {
        nfa: DummyProgram,
    }

    struct DummyExec {
        ro: Arc<DummyExecReadOnly>,
    }

    let capture_index = Arc::new(HashMap::new());
    let nfa = DummyProgram {
        capture_name_idx: capture_index.clone(),
    };
    let ro = Arc::new(DummyExecReadOnly { nfa });
    let exec = DummyExec { ro };

    assert_eq!(exec.capture_name_idx().len(), 0);
}

#[test]
fn test_capture_name_idx_non_empty() {
    struct DummyProgram {
        capture_name_idx: Arc<HashMap<String, usize>>,
    }

    struct DummyExecReadOnly {
        nfa: DummyProgram,
    }

    struct DummyExec {
        ro: Arc<DummyExecReadOnly>,
    }

    let mut capture_index = HashMap::new();
    capture_index.insert("group1".to_string(), 0);
    capture_index.insert("group2".to_string(), 1);
    let capture_index_arc = Arc::new(capture_index);
    let nfa = DummyProgram {
        capture_name_idx: capture_index_arc.clone(),
    };
    let ro = Arc::new(DummyExecReadOnly { nfa });
    let exec = DummyExec { ro };

    let index = exec.capture_name_idx();
    assert_eq!(index.len(), 2);
    assert_eq!(*index["group1"], 0);
    assert_eq!(*index["group2"], 1);
}

