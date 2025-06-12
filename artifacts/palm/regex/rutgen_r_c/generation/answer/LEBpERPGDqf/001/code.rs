// Answer 0

#[test]
fn test_regex_as_str_valid() {
    struct DummyExec {
        ro: Arc<ExecReadOnly>,
    }
    
    impl DummyExec {
        fn new(regex_strings: Vec<String>) -> Self {
            Self {
                ro: Arc::new(ExecReadOnly { res: regex_strings }),
            }
        }
    }

    struct DummyRegex(DummyExec);
    
    impl DummyRegex {
        fn as_str(&self) -> &str {
            &self.0.regex_strings()[0]
        }
    }

    let regex_valid = DummyRegex(DummyExec::new(vec!["test".to_string()]));
    assert_eq!(regex_valid.as_str(), "test");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_regex_as_str_empty() {
    struct DummyExec {
        ro: Arc<ExecReadOnly>,
    }
    
    impl DummyExec {
        fn new(regex_strings: Vec<String>) -> Self {
            Self {
                ro: Arc::new(ExecReadOnly { res: regex_strings }),
            }
        }
    }

    struct DummyRegex(DummyExec);
    
    impl DummyRegex {
        fn as_str(&self) -> &str {
            &self.0.regex_strings()[0]
        }
    }

    let regex_empty = DummyRegex(DummyExec::new(vec![]));
    let _ = regex_empty.as_str(); // This should panic
}

