// Answer 0

#[test]
fn test_capture_names_empty() {
    struct MockCapture {
        names: Vec<String>,
    }

    impl MockCapture {
        fn capture_names(&self) -> &Vec<String> {
            &self.names
        }
    }

    struct MockRegex {
        inner: MockCapture,
    }

    impl MockRegex {
        pub fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.inner.capture_names().iter())
        }
    }

    let regex = MockRegex {
        inner: MockCapture { names: vec![] },
    };
    
    let capture_names = regex.capture_names();
    let names: Vec<_> = capture_names.0.collect();
    assert!(names.is_empty());
}

#[test]
fn test_capture_names_single() {
    struct MockCapture {
        names: Vec<String>,
    }

    impl MockCapture {
        fn capture_names(&self) -> &Vec<String> {
            &self.names
        }
    }

    struct MockRegex {
        inner: MockCapture,
    }

    impl MockRegex {
        pub fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.inner.capture_names().iter())
        }
    }

    let regex = MockRegex {
        inner: MockCapture { names: vec!["name1".to_string()] },
    };
    
    let capture_names = regex.capture_names();
    let names: Vec<_> = capture_names.0.collect();
    assert_eq!(names, vec![&"name1".to_string()]);
}

#[test]
fn test_capture_names_multiple() {
    struct MockCapture {
        names: Vec<String>,
    }

    impl MockCapture {
        fn capture_names(&self) -> &Vec<String> {
            &self.names
        }
    }

    struct MockRegex {
        inner: MockCapture,
    }

    impl MockRegex {
        pub fn capture_names(&self) -> CaptureNames {
            CaptureNames(self.inner.capture_names().iter())
        }
    }

    let regex = MockRegex {
        inner: MockCapture { names: vec!["name1".to_string(), "name2".to_string()] },
    };
    
    let capture_names = regex.capture_names();
    let names: Vec<_> = capture_names.0.collect();
    assert_eq!(names, vec![&"name1".to_string(), &"name2".to_string()]);
}

