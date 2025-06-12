// Answer 0

#[test]
fn test_capture_names_with_named_captures() {
    struct TestProgram {
        captures: Vec<Option<String>>,
    }
    
    impl TestProgram {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }
    }

    let program = TestProgram::new(vec![Some("capture1".to_string()), None, Some("capture2".to_string())]);
    let exec_read_only = ExecReadOnly { 
        res: vec![], 
        nfa: program, 
        dfa: program, 
        dfa_reverse: program, 
        suffixes: LiteralSearcher {},
        match_type: MatchType::default(),
    };
    
    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };

    let captures = exec.capture_names();
    assert_eq!(captures, &[Some("capture1".to_string()), None, Some("capture2".to_string())]);
}

#[test]
fn test_capture_names_with_no_captures() {
    struct TestProgram {
        captures: Vec<Option<String>>,
    }

    impl TestProgram {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }
    }

    let program = TestProgram::new(vec![]);
    let exec_read_only = ExecReadOnly { 
        res: vec![],
        nfa: program, 
        dfa: program, 
        dfa_reverse: program, 
        suffixes: LiteralSearcher {},
        match_type: MatchType::default(),
    };

    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };

    let captures = exec.capture_names();
    assert_eq!(captures, &[]);
}

#[test]
fn test_capture_names_with_only_anonymous_captures() {
    struct TestProgram {
        captures: Vec<Option<String>>,
    }

    impl TestProgram {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }
    }

    let program = TestProgram::new(vec![None, None, None]);
    let exec_read_only = ExecReadOnly { 
        res: vec![], 
        nfa: program, 
        dfa: program, 
        dfa_reverse: program, 
        suffixes: LiteralSearcher {},
        match_type: MatchType::default(),
    };

    let exec = Exec { 
        ro: Arc::new(exec_read_only), 
        cache: CachedThreadLocal::new() 
    };

    let captures = exec.capture_names();
    assert_eq!(captures, &[None, None, None]);
}

