// Answer 0

#[test]
fn test_add_capture_name_success() {
    struct DummyParser {
        capture_names: RefCell<Vec<ast::CaptureName>>,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            // To satisfy trait, we can return a dummy reference; real logic may vary
            unimplemented!()
        }
    }

    let parser = DummyParser {
        capture_names: RefCell::new(vec![]),
    };

    let parser_i = ParserI::new(parser, "");

    let capture_name = ast::CaptureName {
        span: Span { start: 0, end: 1 },
        name: "foo".to_string(),
        index: 0,
    };

    let result = parser_i.add_capture_name(&capture_name);

    assert!(result.is_ok());
}

#[test]
fn test_add_capture_name_duplicate() {
    struct DummyParser {
        capture_names: RefCell<Vec<ast::CaptureName>>,
    }

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            // To satisfy trait, we can return a dummy reference; real logic may vary
            unimplemented!()
        }
    }

    let parser = DummyParser {
        capture_names: RefCell::new(vec![
            ast::CaptureName {
                span: Span { start: 0, end: 1 },
                name: "foo".to_string(),
                index: 0,
            },
        ]),
    };

    let parser_i = ParserI::new(parser, "");

    let capture_name_duplicate = ast::CaptureName {
        span: Span { start: 2, end: 3 },
        name: "foo".to_string(),
        index: 1,
    };

    let result = parser_i.add_capture_name(&capture_name_duplicate);

    assert!(result.is_err());
    if let Err(e) = result {
        match e.kind {
            ast::ErrorKind::GroupNameDuplicate { .. } => {}
            _ => panic!("Expected GroupNameDuplicate error"),
        }
    }
}

