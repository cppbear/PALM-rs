// Answer 0

#[test]
fn test_invalid_uri_parts_display() {
    struct MockInvalidUri;
    
    impl fmt::Display for MockInvalidUri {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "mock invalid URI")
        }
    }

    let invalid_uri = MockInvalidUri;
    let invalid_uri_parts = InvalidUriParts(invalid_uri);
    
    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        invalid_uri_parts.fmt(formatter).unwrap();
    }
    
    assert_eq!(output, "mock invalid URI");
}

#[test]
fn test_invalid_uri_parts_debug() {
    struct MockInvalidUri;

    impl fmt::Debug for MockInvalidUri {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MockInvalidUri")
        }
    }

    let invalid_uri = MockInvalidUri;
    let invalid_uri_parts = InvalidUriParts(invalid_uri);
    
    let debug_output = format!("{:?}", invalid_uri_parts);
    
    assert!(debug_output.contains("MockInvalidUri"));
}

