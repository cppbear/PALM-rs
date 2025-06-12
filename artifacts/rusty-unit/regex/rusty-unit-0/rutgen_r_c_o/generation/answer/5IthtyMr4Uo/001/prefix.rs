// Answer 0

#[test]
fn test_captures_len_empty() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            nfa: NFA {
                captures: vec![]
            }
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    regex.captures_len();
}

#[test]
fn test_captures_len_one() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            nfa: NFA {
                captures: vec![Some("capture1".to_string())]
            }
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    regex.captures_len();
}

#[test]
fn test_captures_len_multiple() {
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            nfa: NFA {
                captures: vec![
                    Some("capture1".to_string()), 
                    Some("capture2".to_string()), 
                    Some("capture3".to_string())
                ]
            }
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    regex.captures_len();
}

#[test]
fn test_captures_len_maximum() {
    let captures = (0..1000).map(|i| Some(format!("capture{}", i))).collect::<Vec<_>>();
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            nfa: NFA {
                captures,
            }
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    regex.captures_len();
}

#[test]
fn test_captures_len_over_maximum() {
    let captures = (0..1001).map(|i| Some(format!("capture{}", i))).collect::<Vec<_>>();
    let exec = Exec {
        ro: Arc::new(ExecReadOnly {
            nfa: NFA {
                captures,
            }
        }),
        cache: CachedThreadLocal::new(),
    };
    let regex = Regex(exec);
    regex.captures_len();
}

