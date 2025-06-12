// Answer 0

#[test]
fn test_capture_names_empty() {
    struct TestExec {
        capture_names: Vec<Option<String>>,
    }
    
    impl TestExec {
        fn new(capture_names: Vec<Option<String>>) -> Self {
            Self { capture_names }
        }
        
        fn capture_names(&self) -> &[Option<String>] {
            &self.capture_names
        }
    }
    
    struct TestRegex(TestExec);

    impl TestRegex {
        fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let exec = TestExec::new(vec![]);
    let regex = TestRegex(exec);
    let names: Vec<Option<String>> = regex.capture_names().0.cloned().collect();
    assert_eq!(names, vec![]);
}

#[test]
fn test_capture_names_single() {
    struct TestExec {
        capture_names: Vec<Option<String>>,
    }
    
    impl TestExec {
        fn new(capture_names: Vec<Option<String>>) -> Self {
            Self { capture_names }
        }
        
        fn capture_names(&self) -> &[Option<String>] {
            &self.capture_names
        }
    }
    
    struct TestRegex(TestExec);

    impl TestRegex {
        fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let exec = TestExec::new(vec![Some("group1".to_string())]);
    let regex = TestRegex(exec);
    let names: Vec<Option<String>> = regex.capture_names().0.cloned().collect();
    assert_eq!(names, vec![Some("group1".to_string())]);
}

#[test]
fn test_capture_names_multiple() {
    struct TestExec {
        capture_names: Vec<Option<String>>,
    }
    
    impl TestExec {
        fn new(capture_names: Vec<Option<String>>) -> Self {
            Self { capture_names }
        }
        
        fn capture_names(&self) -> &[Option<String>] {
            &self.capture_names
        }
    }
    
    struct TestRegex(TestExec);

    impl TestRegex {
        fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let exec = TestExec::new(vec![Some("group1".to_string()), Some("group2".to_string()), None]);
    let regex = TestRegex(exec);
    let names: Vec<Option<String>> = regex.capture_names().0.cloned().collect();
    assert_eq!(names, vec![Some("group1".to_string()), Some("group2".to_string()), None]);
}

