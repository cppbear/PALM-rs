// Answer 0

#[test]
fn test_parse_escape_octal_valid() {
    let pattern = "\\123";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = ParserI {
        parser: Parser { octal: true, ..Default::default() },
        pattern,
    };

    assert_eq!(parser.parse_escape(), Ok(Primitive::Literal(ast::Literal {
        span: Span::new(pos, Position { offset: 3, line: 1, column: 4 }),
        kind: ast::LiteralKind::Octal,
        c: 'S', // Assuming '\123' corresponds to 'S'
    })));
}

#[test]
fn test_parse_escape_octal_exceeding() {
    let pattern = "\\1234";  // Invalid octal since length is > 3
    let pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = ParserI {
        parser: Parser { octal: true, ..Default::default() },
        pattern,
    };

    assert_eq!(parser.parse_escape(), Err(parser.error(Span::new(pos, Position { offset: 4, line: 1, column: 5 }), ast::ErrorKind::EscapeUnexpectedEof)));
}

#[test]
fn test_parse_escape_octal_disabled() {
    let pattern = "\\123";
    let pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = ParserI {
        parser: Parser { octal: false, ..Default::default() },
        pattern,
    };

    assert_eq!(parser.parse_escape(), Err(parser.error(Span::new(pos, Position { offset: 3, line: 1, column: 4 }), ast::ErrorKind::UnsupportedBackreference)));
}

#[test]
fn test_parse_escape_special_character() {
    let pattern = "\\n";  // Newline character 
    let pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = ParserI {
        parser: Parser { octal: true, ..Default::default() },
        pattern,
    };

    assert_eq!(parser.parse_escape(), Ok(Primitive::Literal(ast::Literal {
        span: Span::new(pos, Position { offset: 2, line: 1, column: 3 }),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed),
        c: '\n',
    })));
}

#[test]
fn test_parse_escape_invalid_escape() {
    let pattern = "\\xZ";  // Invalid hex escape sequence
    let pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = ParserI {
        parser: Parser { octal: true, ..Default::default() },
        pattern,
    };

    assert_eq!(parser.parse_escape(), Err(parser.error(Span::new(pos, Position { offset: 3, line: 1, column: 4 }), ast::ErrorKind::EscapeUnrecognized)));
}

