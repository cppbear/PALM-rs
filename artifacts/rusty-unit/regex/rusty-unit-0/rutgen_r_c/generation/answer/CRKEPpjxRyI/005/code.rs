// Answer 0

#[test]
fn test_parse_set_class_range_valid() {
    struct TestParser;
    
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // Provide a mock Parser object if needed.
            unimplemented!()
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: "a-b",
    };
    
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());

    if let Ok(ast::ClassSetItem::Range(range)) = result {
        assert_eq!(range.start.c, 'a');
        assert_eq!(range.end.c, 'b');
    }
}

#[test]
fn test_parse_set_class_range_unclosed() {
    struct TestParser {
        eof: bool,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    impl TestParser {
        fn is_eof(&self) -> bool {
            self.eof
        }
    }
    
    let parser = ParserI {
        parser: TestParser { eof: true },
        pattern: "a-",
    };

    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct TestParser;

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: "b-a", // Invalid range where start > end
    };

    let result = parser.parse_set_class_range();
    assert!(result.is_err());
    // Check specific error kind for ClassRangeInvalid if possible.
}

