// Answer 0

#[test]
fn test_parse_hex_digits_valid_x() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    let mut parser = ParserI {
        parser: Parser { /* initialize necessary fields */ },
        pattern: "\\x4f", // This is the literal we're testing
    };

    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_ok());
    match result {
        Ok(literal) => {
            assert_eq!(literal.c, 'O');
            assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::X));
            assert!(literal.span.start == span_start.start && literal.span.end.offset == position.offset + 3);
        },
        _ => panic!("Expected Ok but got an error."),
    }
}

#[test]
fn test_parse_hex_digits_empty_string() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    let mut parser = ParserI {
        parser: Parser { /* initialize necessary fields */ },
        pattern: "", // Testing with an empty string
    };

    let result = parser.parse_hex_digits(ast::HexLiteralKind::UnicodeShort); // Expecting 4 digits
    assert!(result.is_err());
    match result {
        Err(err) => {
            // Check for the correct error kind
            assert_eq!(err.kind, ast::ErrorKind::EscapeUnexpectedEof);
        },
        _ => panic!("Expected Err but got Ok."),
    }
}

#[test]
fn test_parse_hex_digits_invalid_character() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(position, position);
    let mut parser = ParserI {
        parser: Parser { /* initialize necessary fields */ },
        pattern: "\\xZ4", // Invalid hex character 'Z'
    };

    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_err());
    match result {
        Err(err) => {
            assert_eq!(err.kind, ast::ErrorKind::EscapeHexInvalidDigit);
        },
        _ => panic!("Expected Err but got Ok."),
    }
}

