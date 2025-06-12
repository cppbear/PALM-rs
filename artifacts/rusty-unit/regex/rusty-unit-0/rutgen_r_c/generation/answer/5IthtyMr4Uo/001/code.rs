// Answer 0

#[test]
fn test_captures_len_with_no_captures() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }

    impl TestExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }

    struct TestRegex(Arc<TestExec>);

    impl TestRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec = Arc::new(TestExec::new(vec![]));
    let regex = TestRegex(exec);
    assert_eq!(regex.captures_len(), 0);
}

#[test]
fn test_captures_len_with_multiple_captures() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }

    impl TestExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }

    struct TestRegex(Arc<TestExec>);

    impl TestRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec = Arc::new(TestExec::new(vec![Some("capture1".to_string()), Some("capture2".to_string())]));
    let regex = TestRegex(exec);
    assert_eq!(regex.captures_len(), 2);
}

#[test]
fn test_captures_len_with_none_captures() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }

    impl TestExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }

    struct TestRegex(Arc<TestExec>);

    impl TestRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec = Arc::new(TestExec::new(vec![None, None]));
    let regex = TestRegex(exec);
    assert_eq!(regex.captures_len(), 2);
}

#[test]
fn test_captures_len_with_mixed_captures() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }

    impl TestExec {
        fn new(captures: Vec<Option<String>>) -> Self {
            Self { captures }
        }

        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }

    struct TestRegex(Arc<TestExec>);

    impl TestRegex {
        fn captures_len(&self) -> usize {
            self.0.capture_names().len()
        }
    }

    let exec = Arc::new(TestExec::new(vec![Some("capture1".to_string()), None, Some("capture3".to_string())]));
    let regex = TestRegex(exec);
    assert_eq!(regex.captures_len(), 3);
}

