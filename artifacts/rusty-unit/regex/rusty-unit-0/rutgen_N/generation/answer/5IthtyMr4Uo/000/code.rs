// Answer 0

#[test]
fn test_captures_len_empty() {
    struct CaptureGroup {
        names: Vec<Option<String>>,
    }

    impl CaptureGroup {
        fn capture_names(&self) -> &Vec<Option<String>> {
            &self.names
        }
    }

    let group = CaptureGroup { names: vec![] };
    assert_eq!(group.capture_names().len(), 0);
}

#[test]
fn test_captures_len_single_capture() {
    struct CaptureGroup {
        names: Vec<Option<String>>,
    }

    impl CaptureGroup {
        fn capture_names(&self) -> &Vec<Option<String>> {
            &self.names
        }
    }

    let group = CaptureGroup { names: vec![Some("first".to_string())] };
    assert_eq!(group.capture_names().len(), 1);
}

#[test]
fn test_captures_len_multiple_captures() {
    struct CaptureGroup {
        names: Vec<Option<String>>,
    }

    impl CaptureGroup {
        fn capture_names(&self) -> &Vec<Option<String>> {
            &self.names
        }
    }

    let group = CaptureGroup { names: vec![Some("first".to_string()), Some("second".to_string()), None] };
    assert_eq!(group.capture_names().len(), 3);
}

