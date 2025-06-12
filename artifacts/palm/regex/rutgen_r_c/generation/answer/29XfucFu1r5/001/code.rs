// Answer 0

#[test]
fn test_into_class_set_item_invalid_assertion() {
    struct TestParser;
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // Return a dummy Parser
            &Parser {}
        }
    }

    let parser_instance = ParserI::new(TestParser, "test_pattern");
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Primitive::Assertion(Assertion { span, kind: AssertionKind::WordBoundary });

    let result = assertion.into_class_set_item(&parser_instance);

    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassEscapeInvalid);
        assert_eq!(err.pattern, "test_pattern");
        assert_eq!(err.span, span);
    } else {
        panic!("Expected an error but got a successful result.");
    }
}

#[test]
fn test_into_class_set_item_invalid_dot() {
    struct TestParser;
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // Return a dummy Parser
            &Parser {}
        }
    }

    let parser_instance = ParserI::new(TestParser, "test_pattern");
    let span = Span { start: Position(0), end: Position(1) };
    let dot = Primitive::Dot(span);

    let result = dot.into_class_set_item(&parser_instance);

    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassEscapeInvalid);
        assert_eq!(err.pattern, "test_pattern");
        assert_eq!(err.span, span);
    } else {
        panic!("Expected an error but got a successful result.");
    }
}

