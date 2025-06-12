// Answer 0

#[test]
fn test_expand_str_basic() {
    use re_unicode::{Captures, Ref};

    struct TestCaptures {
        data: Vec<Option<String>>,
    }

    impl Captures for TestCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).map(|s| s.as_ref().map_or("", String::as_str)).unwrap_or("")
        }

        fn name(&self, _name: &str) -> Option<&str> {
            Some("named_capture")
        }
    }

    let mut caps = TestCaptures {
        data: vec![Some("first".to_string()), Some("second".to_string())],
    };

    let mut dst = String::new();
    let replacement = "Captured: $0, Named: $named_capture";

    expand_str(&caps, replacement, &mut dst);
    
    assert_eq!(dst, "Captured: first, Named: named_capture");
}

#[test]
fn test_expand_str_edge_case_double_dollar() {
    use re_unicode::{Captures, Ref};

    struct TestCaptures {
        data: Vec<Option<String>>,
    }

    impl Captures for TestCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).map(|s| s.as_ref().map_or("", String::as_str)).unwrap_or("")
        }

        fn name(&self, _name: &str) -> Option<&str> {
            Some("named_capture")
        }
    }

    let mut caps = TestCaptures {
        data: vec![Some("first".to_string()), Some("second".to_string())],
    };

    let mut dst = String::new();
    let replacement = "Value: $$";

    expand_str(&caps, replacement, &mut dst);
    
    assert_eq!(dst, "Value: $");
}

#[test]
fn test_expand_str_empty_replacement() {
    use re_unicode::{Captures, Ref};

    struct TestCaptures {
        data: Vec<Option<String>>,
    }

    impl Captures for TestCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).map(|s| s.as_ref().map_or("", String::as_str)).unwrap_or("")
        }

        fn name(&self, _name: &str) -> Option<&str> {
            Some("named_capture")
        }
    }

    let mut caps = TestCaptures {
        data: vec![Some("first".to_string())],
    };

    let mut dst = String::new();
    let replacement = "Value: $0, Named: $named_capture";
    
    expand_str(&caps, replacement, &mut dst);

    assert_eq!(dst, "Value: first, Named: named_capture");
}

