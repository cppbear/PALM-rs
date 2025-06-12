// Answer 0

#[test]
fn test_capture_name_idx() {
    use std::collections::HashMap;
    use std::sync::Arc;

    // Creating a simple test case for ExecReadOnly
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("group1".to_string(), 0);
    capture_name_idx.insert("group2".to_string(), 1);

    let program = Program {}; // Assuming a default constructor exists for Program

    let nfa = ExecReadOnly {
        res: vec!["test_regex".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher {}, // Assuming a default constructor exists
        match_type: MatchType {},      // Assuming a default constructor exists or suitable initial value
    };

    // Setting the capture_name_idx field in an Arc
    let ro = Arc::new(nfa);
    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &RefCell::new(ProgramCacheInner {}), // Assuming a default constructor exists
    };

    let capture_idx = exec_no_sync.capture_name_idx();
    assert_eq!(capture_idx.len(), 2);
    assert_eq!(capture_idx.get("group1"), Some(&0));
    assert_eq!(capture_idx.get("group2"), Some(&1));
}

#[test]
fn test_capture_name_idx_empty() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let capture_name_idx = HashMap::new();

    let program = Program {}; // Assuming a default constructor exists for Program

    let nfa = ExecReadOnly {
        res: vec![],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher {}, // Assuming a default constructor exists
        match_type: MatchType {},      // Assuming a default constructor exists or suitable initial value
    };

    let ro = Arc::new(nfa);
    let exec_no_sync = ExecNoSync {
        ro: &ro,
        cache: &RefCell::new(ProgramCacheInner {}), // Assuming a default constructor exists
    };

    let capture_idx = exec_no_sync.capture_name_idx();
    assert_eq!(capture_idx.len(), 0);
}

