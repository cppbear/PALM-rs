// Answer 0

#[test]
fn test_captures_len_empty() {
    struct MockExecReadOnly {
        captures: Vec<Option<String>>,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    impl MockExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.captures
        }
    }

    struct MockRegex(MockExec);

    impl MockRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec_ro = MockExecReadOnly {
        captures: Vec::new(),
    };
    let mock_exec = MockExec {
        ro: Arc::new(exec_ro),
    };
    let regex = MockRegex(mock_exec);

    assert_eq!(regex.captures_len(), 0);
}

#[test]
fn test_captures_len_single_capture() {
    struct MockExecReadOnly {
        captures: Vec<Option<String>>,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    impl MockExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.captures
        }
    }

    struct MockRegex(MockExec);

    impl MockRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec_ro = MockExecReadOnly {
        captures: vec![Some("first_capture".to_string())],
    };
    let mock_exec = MockExec {
        ro: Arc::new(exec_ro),
    };
    let regex = MockRegex(mock_exec);

    assert_eq!(regex.captures_len(), 1);
}

#[test]
fn test_captures_len_multiple_captures() {
    struct MockExecReadOnly {
        captures: Vec<Option<String>>,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    impl MockExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.captures
        }
    }

    struct MockRegex(MockExec);

    impl MockRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec_ro = MockExecReadOnly {
        captures: vec![
            Some("first_capture".to_string()),
            Some("second_capture".to_string()),
            None,
            Some("third_capture".to_string()),
        ],
    };
    let mock_exec = MockExec {
        ro: Arc::new(exec_ro),
    };
    let regex = MockRegex(mock_exec);

    assert_eq!(regex.captures_len(), 4);
}

#[test]
fn test_captures_len_no_captures() {
    struct MockExecReadOnly {
        captures: Vec<Option<String>>,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    impl MockExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.captures
        }
    }

    struct MockRegex(MockExec);

    impl MockRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec_ro = MockExecReadOnly {
        captures: vec![None, None, None],
    };
    let mock_exec = MockExec {
        ro: Arc::new(exec_ro),
    };
    let regex = MockRegex(mock_exec);

    assert_eq!(regex.captures_len(), 3);
}

