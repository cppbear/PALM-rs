// Answer 0

#[test]
fn test_add_capture_name_unique() {
    struct MockParser {
        capture_names: RefCell<Vec<ast::CaptureName>>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let mut mock_parser = MockParser {
        capture_names: RefCell::new(vec![]),
    };

    let capture_name = ast::CaptureName {
        span: ast::Span { start: 0, end: 1 },
        name: "unique_capture".to_string(),
        index: 0,
    };

    let parser_i = ParserI::new(&mock_parser, "pattern");

    let result = parser_i.add_capture_name(&capture_name);
    assert!(result.is_ok());
}

#[test]
fn test_add_capture_name_duplicate() {
    struct MockParser {
        capture_names: RefCell<Vec<ast::CaptureName>>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let mut mock_parser = MockParser {
        capture_names: RefCell::new(vec![
            ast::CaptureName {
                span: ast::Span { start: 0, end: 1 },
                name: "existing_capture".to_string(),
                index: 0,
            },
        ]),
    };

    let capture_name = ast::CaptureName {
        span: ast::Span { start: 0, end: 1 },
        name: "existing_capture".to_string(),
        index: 1,
    };

    let parser_i = ParserI::new(&mock_parser, "pattern");

    let result = parser_i.add_capture_name(&capture_name);
    assert!(result.is_err());
}

#[test]
fn test_add_capture_name_empty() {
    struct MockParser {
        capture_names: RefCell<Vec<ast::CaptureName>>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let mut mock_parser = MockParser {
        capture_names: RefCell::new(vec![]),
    };

    let capture_name = ast::CaptureName {
        span: ast::Span { start: 0, end: 1 },
        name: "".to_string(),
        index: 0,
    };

    let parser_i = ParserI::new(&mock_parser, "pattern");

    let result = parser_i.add_capture_name(&capture_name);
    assert!(result.is_err());
}

