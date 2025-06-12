// Answer 0

#[test]
fn test_capture_names_with_none_names() {
    struct ExecReadOnly {
        captures: Vec<Option<String>>,
    }
    
    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    struct Regex(Exec);
    
    impl Regex {
        pub fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let captures = vec![None, None, None];
    let exec_read_only = ExecReadOnly { captures };
    let exec = Exec { ro: Arc::new(exec_read_only) };
    let regex = Regex(exec);
    
    let capture_names_iter = regex.capture_names();
    let result: Vec<Option<String>> = capture_names_iter.0.cloned().collect();
    
    assert_eq!(result, vec![None, None, None]);
}

#[test]
fn test_capture_names_with_some_names() {
    struct ExecReadOnly {
        captures: Vec<Option<String>>,
    }
    
    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    struct Regex(Exec);
    
    impl Regex {
        pub fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let captures = vec![Some("first".to_string()), Some("second".to_string()), None];
    let exec_read_only = ExecReadOnly { captures };
    let exec = Exec { ro: Arc::new(exec_read_only) };
    let regex = Regex(exec);
    
    let capture_names_iter = regex.capture_names();
    let result: Vec<Option<String>> = capture_names_iter.0.cloned().collect();
    
    assert_eq!(result, vec![Some("first".to_string()), Some("second".to_string()), None]);
}

#[test]
fn test_capture_names_empty() {
    struct ExecReadOnly {
        captures: Vec<Option<String>>,
    }
    
    struct Exec {
        ro: Arc<ExecReadOnly>,
    }

    struct Regex(Exec);
    
    impl Regex {
        pub fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.0.capture_names().iter())
        }
    }

    let captures: Vec<Option<String>> = vec![];
    let exec_read_only = ExecReadOnly { captures };
    let exec = Exec { ro: Arc::new(exec_read_only) };
    let regex = Regex(exec);
    
    let capture_names_iter = regex.capture_names();
    let result: Vec<Option<String>> = capture_names_iter.0.cloned().collect();
    
    assert_eq!(result, vec![]);
}

