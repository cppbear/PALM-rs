// Answer 0

#[test]
fn test_capture_names_empty() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }
    
    impl TestExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }
    
    struct TestRegex(TestExec);
    
    impl TestRegex {
        fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let test_exec = TestExec { captures: Vec::new() };
    let test_regex = TestRegex(test_exec);
    
    let capture_names_iter = test_regex.capture_names();
    let captured_names: Vec<_> = capture_names_iter.0.cloned().collect();
    
    assert_eq!(captured_names, Vec::<Option<String>>::new());
}

#[test]
fn test_capture_names_single() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }
    
    impl TestExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }
    
    struct TestRegex(TestExec);
    
    impl TestRegex {
        fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let test_exec = TestExec { captures: vec![Some("name1".to_string())] };
    let test_regex = TestRegex(test_exec);
    
    let capture_names_iter = test_regex.capture_names();
    let captured_names: Vec<_> = capture_names_iter.0.cloned().collect();
    
    assert_eq!(captured_names, vec![Some("name1".to_string())]);
}

#[test]
fn test_capture_names_multiple() {
    struct TestExec {
        captures: Vec<Option<String>>,
    }
    
    impl TestExec {
        fn capture_names(&self) -> &[Option<String>] {
            &self.captures
        }
    }
    
    struct TestRegex(TestExec);
    
    impl TestRegex {
        fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let test_exec = TestExec { captures: vec![Some("name1".to_string()), Some("name2".to_string()), None] };
    let test_regex = TestRegex(test_exec);
    
    let capture_names_iter = test_regex.capture_names();
    let captured_names: Vec<_> = capture_names_iter.0.cloned().collect();
    
    assert_eq!(captured_names, vec![Some("name1".to_string()), Some("name2".to_string()), None]);
}

