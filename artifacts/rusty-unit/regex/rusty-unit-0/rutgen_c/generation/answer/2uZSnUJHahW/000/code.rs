// Answer 0

#[test]
fn test_parse_hex_brace_valid() {
    struct ParserBorrow {
        scratch: RefCell<String>,
    }

    impl Borrow<Parser> for ParserBorrow {
        fn borrow(&self) -> &Parser {
            unimplemented!() // Implement appropriate parser behavior here
        }
    }

    let parser = ParserBorrow {
        scratch: RefCell::new(String::from("61}")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{61}",
    };

    let expected_literal = ast::Literal {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }),
        kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
        c: 'a',
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    assert_eq!(result.ok(), Some(expected_literal));
}

#[test]
fn test_parse_hex_brace_empty() {
    struct ParserBorrow {
        scratch: RefCell<String>,
    }

    impl Borrow<Parser> for ParserBorrow {
        fn borrow(&self) -> &Parser {
            unimplemented!() // Implement appropriate parser behavior here
        }
    }

    let parser = ParserBorrow {
        scratch: RefCell::new(String::from("}")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{}",
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_unexpected_eof() {
    struct ParserBorrow {
        scratch: RefCell<String>,
    }

    impl Borrow<Parser> for ParserBorrow {
        fn borrow(&self) -> &Parser {
            unimplemented!() // Implement appropriate parser behavior here
        }
    }

    let parser = ParserBorrow {
        scratch: RefCell::new(String::from("61")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{61",
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_invalid_character() {
    struct ParserBorrow {
        scratch: RefCell<String>,
    }

    impl Borrow<Parser> for ParserBorrow {
        fn borrow(&self) -> &Parser {
            unimplemented!() // Implement appropriate parser behavior here
        }
    }

    let parser = ParserBorrow {
        scratch: RefCell::new(String::from("G}")),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{G}",
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    assert!(result.is_err());
}

