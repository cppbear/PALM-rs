// Answer 0

#[test]
fn test_captures_len_no_captures() {
    struct MockExecReadOnly {
        captures: Vec<Option<String>>,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    impl MockExec {
        fn new() -> Self {
            Self {
                ro: Arc::new(MockExecReadOnly {
                    captures: Vec::new(),
                }),
            }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.captures
        }
    }

    struct MockRegex(MockExec);

    impl MockRegex {
        fn new() -> Self {
            Self(MockExec::new())
        }

        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let regex = MockRegex::new();
    assert_eq!(regex.captures_len(), 0);
}

#[test]
fn test_captures_len_with_captures() {
    struct MockExecReadOnly {
        captures: Vec<Option<String>>,
    }

    struct MockExec {
        ro: Arc<MockExecReadOnly>,
    }

    impl MockExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self {
                ro: Arc::new(MockExecReadOnly { captures }),
            }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.ro.captures
        }
    }

    struct MockRegex(MockExec);

    impl MockRegex {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self(MockExec::new(captures))
        }

        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let regex = MockRegex::new(vec![Some("first".to_string()), Some("second".to_string())]);
    assert_eq!(regex.captures_len(), 2);
}

