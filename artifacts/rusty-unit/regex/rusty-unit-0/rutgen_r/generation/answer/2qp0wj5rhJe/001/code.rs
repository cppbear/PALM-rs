// Answer 0

#[test]
fn test_captures_len_with_multiple_captures() {
    struct TestCaptures(Vec<Option<&'static str>>);
    
    impl TestCaptures {
        fn capture_names(&self) -> &Vec<Option<&'static str>> {
            &self.0
        }
    }

    let captures = TestCaptures(vec![Some("group1"), Some("group2"), Some("group3")]);
    assert_eq!(captures_len(&captures), 3);
}

#[test]
fn test_captures_len_with_no_captures() {
    struct TestCaptures(Vec<Option<&'static str>>);
    
    impl TestCaptures {
        fn capture_names(&self) -> &Vec<Option<&'static str>> {
            &self.0
        }
    }

    let captures = TestCaptures(vec![]);
    assert_eq!(captures_len(&captures), 0);
}

#[test]
fn test_captures_len_with_only_empty_captures() {
    struct TestCaptures(Vec<Option<&'static str>>);
    
    impl TestCaptures {
        fn capture_names(&self) -> &Vec<Option<&'static str>> {
            &self.0
        }
    }

    let captures = TestCaptures(vec![None, None, None]);
    assert_eq!(captures_len(&captures), 3);
}

#[test]
fn test_captures_len_with_mixed_empty_and_non_empty() {
    struct TestCaptures(Vec<Option<&'static str>>);
    
    impl TestCaptures {
        fn capture_names(&self) -> &Vec<Option<&'static str>> {
            &self.0
        }
    }

    let captures = TestCaptures(vec![Some("group1"), None, Some("group2")]);
    assert_eq!(captures_len(&captures), 3);
}

