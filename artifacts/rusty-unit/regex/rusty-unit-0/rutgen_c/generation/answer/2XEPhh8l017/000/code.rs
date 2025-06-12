// Answer 0

#[test]
fn test_parse_hex_digits_valid_x() {
    let pattern = "\\x61"; // represents 'a'
    let parser = ParserI {
        parser: Parser {
            ast: ast::parse::Parser {},
            hir: hir::translate::Translator {},
        },
        pattern,
    };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.c, 'a');
    assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::X));
}

#[test]
fn test_parse_hex_digits_valid_unicode_short() {
    let pattern = "\\u0061"; // represents 'a'
    let parser = ParserI {
        parser: Parser {
            ast: ast::parse::Parser {},
            hir: hir::translate::Translator {},
        },
        pattern,
    };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::UnicodeShort);
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.c, 'a');
    assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort));
}

#[test]
fn test_parse_hex_digits_valid_unicode_long() {
    let pattern = "\\U00000061"; // represents 'a'
    let parser = ParserI {
        parser: Parser {
            ast: ast::parse::Parser {},
            hir: hir::translate::Translator {},
        },
        pattern,
    };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::UnicodeLong);
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.c, 'a');
    assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeLong));
}

#[test]
fn test_parse_hex_digits_invalid_digit() {
    let pattern = "\\x6G"; // 'G' is not a valid hex digit
    let parser = ParserI {
        parser: Parser {
            ast: ast::parse::Parser {},
            hir: hir::translate::Translator {},
        },
        pattern,
    };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_digits_unexpected_eof() {
    let pattern = "\\x"; // insufficient digits
    let parser = ParserI {
        parser: Parser {
            ast: ast::parse::Parser {},
            hir: hir::translate::Translator {},
        },
        pattern,
    };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_err());
}

