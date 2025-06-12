// Answer 0

#[test]
fn test_capture_names_empty() {
    struct TestProgram {
        captures: Vec<Option<String>>,
    }

    let test_exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program {
                captures: vec![],
                // ... initialize other fields as needed
            },
            dfa: Program { captures: vec![] },
            dfa_reverse: Program { captures: vec![] },
            suffixes: LiteralSearcher {},
            match_type: MatchType {},
        }),
        cache: CachedThreadLocal::new(),
    };

    let result = test_exec.capture_names();
    assert_eq!(result, &[]);
}

#[test]
fn test_capture_names_one_named() {
    struct TestProgram {
        captures: Vec<Option<String>>,
    }

    let test_exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program {
                captures: vec![Some("name1".to_string())],
                // ... initialize other fields as needed
            },
            dfa: Program { captures: vec![] },
            dfa_reverse: Program { captures: vec![] },
            suffixes: LiteralSearcher {},
            match_type: MatchType {},
        }),
        cache: CachedThreadLocal::new(),
    };

    let result = test_exec.capture_names();
    assert_eq!(result, &[Some("name1".to_string())]);
}

#[test]
fn test_capture_names_multiple_named_and_unnamed() {
    struct TestProgram {
        captures: Vec<Option<String>>,
    }

    let test_exec = Exec {
        ro: Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program {
                captures: vec![Some("name1".to_string()), None, Some("name2".to_string())],
                // ... initialize other fields as needed
            },
            dfa: Program { captures: vec![] },
            dfa_reverse: Program { captures: vec![] },
            suffixes: LiteralSearcher {},
            match_type: MatchType {},
        }),
        cache: CachedThreadLocal::new(),
    };

    let result = test_exec.capture_names();
    assert_eq!(result, &[Some("name1".to_string()), None, Some("name2".to_string())]);
}

