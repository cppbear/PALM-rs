// Answer 0

#[test]
fn test_capture_names_empty() {
    struct TestExec {
        ro: Arc<ExecReadOnly>,
        cache: CachedThreadLocal<ProgramCache>,
    }

    impl TestExec {
        fn new() -> Self {
            TestExec {
                ro: Arc::new(ExecReadOnly {
                    nfa: NFA { captures: Vec::new() },
                }),
                cache: CachedThreadLocal::new(),
            }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.nfa.captures
        }
    }

    let exec = TestExec::new();
    let regex = Regex(exec);
    
    let capture_names: Vec<_> = regex.capture_names().collect();
    
    assert_eq!(capture_names.len(), 0);
}

#[test]
fn test_capture_names_single() {
    struct TestExec {
        ro: Arc<ExecReadOnly>,
        cache: CachedThreadLocal<ProgramCache>,
    }

    impl TestExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            TestExec {
                ro: Arc::new(ExecReadOnly {
                    nfa: NFA { captures },
                }),
                cache: CachedThreadLocal::new(),
            }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.nfa.captures
        }
    }

    let exec = TestExec::new(vec![Some("name".to_string())]);
    let regex = Regex(exec);
    
    let capture_names: Vec<_> = regex.capture_names().collect();
    
    assert_eq!(capture_names.len(), 1);
    assert_eq!(capture_names[0], &Some("name".to_string()));
}

#[test]
fn test_capture_names_multiple() {
    struct TestExec {
        ro: Arc<ExecReadOnly>,
        cache: CachedThreadLocal<ProgramCache>,
    }

    impl TestExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            TestExec {
                ro: Arc::new(ExecReadOnly {
                    nfa: NFA { captures },
                }),
                cache: CachedThreadLocal::new(),
            }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.nfa.captures
        }
    }

    let exec = TestExec::new(vec![Some("first".to_string()), Some("second".to_string())]);
    let regex = Regex(exec);
    
    let capture_names: Vec<_> = regex.capture_names().collect();
    
    assert_eq!(capture_names.len(), 2);
    assert_eq!(capture_names[0], &Some("first".to_string()));
    assert_eq!(capture_names[1], &Some("second".to_string()));
}

