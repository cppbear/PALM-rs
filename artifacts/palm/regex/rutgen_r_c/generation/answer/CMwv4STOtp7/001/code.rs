// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    struct TestParser {
        ignore_whitespace: Cell<bool>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                ignore_whitespace: Cell::new(true),
                ..Default::default()
            }
        }
    }

    let test_parser = TestParser {
        ignore_whitespace: Cell::new(true),
    };
    let parser_i = ParserI::new(test_parser, "test pattern");
    assert_eq!(parser_i.ignore_whitespace(), true);
}

#[test]
fn test_ignore_whitespace_false() {
    struct TestParser {
        ignore_whitespace: Cell<bool>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                ignore_whitespace: Cell::new(false),
                ..Default::default()
            }
        }
    }

    let test_parser = TestParser {
        ignore_whitespace: Cell::new(false),
    };
    let parser_i = ParserI::new(test_parser, "test pattern");
    assert_eq!(parser_i.ignore_whitespace(), false);
}

