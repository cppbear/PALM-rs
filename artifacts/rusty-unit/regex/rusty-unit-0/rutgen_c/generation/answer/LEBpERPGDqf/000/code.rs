// Answer 0

#[test]
fn test_regex_as_str() {
    struct MockExecReadOnly {
        res: Vec<String>,
    }
    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }
    impl MockExec {
        fn new(res: Vec<String>) -> Self {
            MockExec {
                ro: Arc::new(MockExecReadOnly { res }),
            }
        }

        fn regex_strings(&self) -> &[String] {
            &self.ro.res
        }
    }
    
    struct MockRegex(MockExec);

    impl MockRegex {
        fn as_str(&self) -> &str {
            &self.0.regex_strings()[0]
        }
    }

    let mock_exec = MockExec::new(vec![String::from("mock_regex")]);
    let regex = MockRegex(mock_exec);
    assert_eq!(regex.as_str(), "mock_regex");
}

#[test]
fn test_regex_as_str_empty() {
    struct MockExecReadOnly {
        res: Vec<String>,
    }
    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }
    impl MockExec {
        fn new(res: Vec<String>) -> Self {
            MockExec {
                ro: Arc::new(MockExecReadOnly { res }),
            }
        }

        fn regex_strings(&self) -> &[String] {
            &self.ro.res
        }
    }
    
    struct MockRegex(MockExec);

    impl MockRegex {
        fn as_str(&self) -> &str {
            &self.0.regex_strings()[0]
        }
    }

    let mock_exec = MockExec::new(vec![String::from("")]);
    let regex = MockRegex(mock_exec);
    assert_eq!(regex.as_str(), "");
}

