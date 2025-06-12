// Answer 0

#[test]
fn test_parse_str_valid() {
    struct TestParser;

    impl TestParser {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            // mocked implementation
            *scratch = b"test".to_vec(); // Simulate filling the scratch buffer
            Ok(Reference::from_str("test"))
        }
    }

    let mut parser = TestParser;
    let mut scratch = Vec::new();
    let result = parser.parse_str(&mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, b"test");
}

#[test]
fn test_parse_str_empty_scratch() {
    struct TestParser;

    impl TestParser {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            *scratch = Vec::new(); // Empty scratch
            Err("Failed to parse".into())
        }
    }

    let mut parser = TestParser;
    let mut scratch = Vec::new();
    let result = parser.parse_str(&mut scratch);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_str_large_scratch() {
    struct TestParser;

    impl TestParser {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            *scratch = b"large test string for the buffer".to_vec(); // Simulate filling the scratch buffer
            Ok(Reference::from_str("large test string for the buffer"))
        }
    }

    let mut parser = TestParser;
    let mut scratch = Vec::new();
    let result = parser.parse_str(&mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, b"large test string for the buffer");
}

