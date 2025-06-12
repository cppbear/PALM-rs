// Answer 0

#[test]
fn test_parse_valid_pattern() {
    struct TestParser;

    impl TestParser {
        fn new() -> Self {
            TestParser
        }
    }

    let mut parser = TestParser::new();
    let result = parser.parse("a*b+");
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "invalid pattern")]
fn test_parse_invalid_pattern() {
    struct TestParser;

    impl TestParser {
        fn new() -> Self {
            TestParser
        }
    }

    let mut parser = TestParser::new();
    let result = parser.parse("[");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_pattern() {
    struct TestParser;

    impl TestParser {
        fn new() -> Self {
            TestParser
        }
    }

    let mut parser = TestParser::new();
    let result = parser.parse("");
    assert!(result.is_ok());
}

