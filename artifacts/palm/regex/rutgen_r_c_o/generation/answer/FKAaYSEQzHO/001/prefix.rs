// Answer 0

#[test]
fn test_capture_names_empty() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program {
                captures: vec![],
                // Initialize other fields as necessary
            },
            dfa: Program {
                captures: vec![],
            },
            dfa_reverse: Program {
                captures: vec![],
            },
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    let _ = exec.capture_names();
}

#[test]
fn test_capture_names_single_none() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec!["test".to_string()],
            nfa: Program {
                captures: vec![None],
            },
            dfa: Program {
                captures: vec![None],
            },
            dfa_reverse: Program {
                captures: vec![None],
            },
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    let _ = exec.capture_names();
}

#[test]
fn test_capture_names_single_valid() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec!["test".to_string()],
            nfa: Program {
                captures: vec![Some("capture1".to_string())],
            },
            dfa: Program {
                captures: vec![Some("capture1".to_string())],
            },
            dfa_reverse: Program {
                captures: vec![Some("capture1".to_string())],
            },
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    let _ = exec.capture_names();
}

#[test]
fn test_capture_names_multiple() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec!["test".to_string()],
            nfa: Program {
                captures: vec![
                    Some("capture1".to_string()),
                    None,
                    Some("capture2".to_string()),
                ],
            },
            dfa: Program {
                captures: vec![
                    Some("capture1".to_string()),
                    None,
                    Some("capture2".to_string()),
                ],
            },
            dfa_reverse: Program {
                captures: vec![
                    Some("capture1".to_string()),
                    None,
                    Some("capture2".to_string()),
                ],
            },
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    let _ = exec.capture_names();
}

#[test]
fn test_capture_names_large() {
    let captures: Vec<Option<String>> = (0..1000).map(|i| {
        if i % 2 == 0 {
            Some(format!("capture{}", i))
        } else {
            None
        }
    }).collect();
    
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec!["test".to_string()],
            nfa: Program {
                captures,
            },
            dfa: Program {
                captures: vec![],
            },
            dfa_reverse: Program {
                captures: vec![],
            },
            suffixes: LiteralSearcher::new(),
            match_type: MatchType::default(),
        }),
        cache: CachedThreadLocal::new(),
    };
    let _ = exec.capture_names();
}

