// Answer 0

#[test]
fn test_capture_name_idx_empty() {
    let capture_name_idx = Arc::new(HashMap::new());
    let exec_ro = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),  // Assume default initializes required nfa.
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let _ = exec.capture_name_idx();
}

#[test]
fn test_capture_name_idx_single_entry() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("first".to_string(), 0);
    let capture_name_idx = Arc::new(capture_name_idx);
    let exec_ro = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let _ = exec.capture_name_idx();
}

#[test]
fn test_capture_name_idx_multiple_entries() {
    let mut capture_name_idx = HashMap::new();
    capture_name_idx.insert("first".to_string(), 0);
    capture_name_idx.insert("second".to_string(), 1);
    capture_name_idx.insert("third".to_string(), 2);
    let capture_name_idx = Arc::new(capture_name_idx);
    let exec_ro = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let _ = exec.capture_name_idx();
}

#[test]
fn test_capture_name_idx_max_entries() {
    let mut capture_name_idx = HashMap::new();
    for i in 0..1000 {
        capture_name_idx.insert(format!("name{}", i), i);
    }
    let capture_name_idx = Arc::new(capture_name_idx);
    let exec_ro = ExecReadOnly {
        res: vec!["example".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    };
    let exec = ExecNoSync {
        ro: &Arc::new(exec_ro),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    let _ = exec.capture_name_idx();
}

