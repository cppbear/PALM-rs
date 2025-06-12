// Answer 0

#[test]
fn test_captures_len() {
    struct MockCaptures(usize);

    impl MockCaptures {
        fn capture_names(&self) -> Vec<Option<String>> {
            vec![Some("first".to_string()), Some("second".to_string())]
        }
    }

    let captures = MockCaptures(2);
    assert_eq!(captures.captures_len(), 2);
}

#[test]
fn test_captures_len_empty() {
    struct EmptyMockCaptures;

    impl EmptyMockCaptures {
        fn capture_names(&self) -> Vec<Option<String>> {
            vec![]
        }
    }
    
    let empty_captures = EmptyMockCaptures;
    assert_eq!(empty_captures.captures_len(), 0);
}

