// Answer 0

#[test]
fn test_parse_hex_digits_valid_x() {
    let parser = ParserI {
        parser: Parser { 
            ast: ast::parse::Parser { /* fields */ },
            hir: hir::translate::Translator { /* fields */ }
        },
        pattern: "\\x61", // represents 'a' in hex
    };
    
    let result = parser.parse_hex_digits(HexLiteralKind::X);
    assert!(result.is_ok());

    if let Ok(literal) = result {
        assert_eq!(literal.kind, LiteralKind::HexFixed(HexLiteralKind::X));
        assert_eq!(literal.c, 'a');
    }
}

#[test]
fn test_parse_hex_digits_valid_unicode_short() {
    let parser = ParserI {
        parser: Parser { 
            ast: ast::parse::Parser { /* fields */ },
            hir: hir::translate::Translator { /* fields */ }
        },
        pattern: "\\u0061", // represents 'a' in unicode short
    };

    let result = parser.parse_hex_digits(HexLiteralKind::UnicodeShort);
    assert!(result.is_ok());

    if let Ok(literal) = result {
        assert_eq!(literal.kind, LiteralKind::HexFixed(HexLiteralKind::UnicodeShort));
        assert_eq!(literal.c, 'a');
    }
}

#[test]
fn test_parse_hex_digits_valid_unicode_long() {
    let parser = ParserI {
        parser: Parser { 
            ast: ast::parse::Parser { /* fields */ },
            hir: hir::translate::Translator { /* fields */ }
        },
        pattern: "\\U00000061", // represents 'a' in unicode long
    };

    let result = parser.parse_hex_digits(HexLiteralKind::UnicodeLong);
    assert!(result.is_ok());

    if let Ok(literal) = result {
        assert_eq!(literal.kind, LiteralKind::HexFixed(HexLiteralKind::UnicodeLong));
        assert_eq!(literal.c, 'a');
    }
}

#[test]
#[should_panic]
fn test_parse_hex_digits_invalid_character() {
    let parser = ParserI {
        parser: Parser { 
            ast: ast::parse::Parser { /* fields */ },
            hir: hir::translate::Translator { /* fields */ }
        },
        pattern: "\\xzz", // invalid hex characters
    };

    let _ = parser.parse_hex_digits(HexLiteralKind::X);
}

#[test]
fn test_parse_hex_digits_eof_handling() {
    let parser = ParserI {
        parser: Parser { 
            ast: ast::parse::Parser { /* fields */ },
            hir: hir::translate::Translator { /* fields */ }
        },
        pattern: "\\u00", // incomplete unicode short
    };

    let result = parser.parse_hex_digits(HexLiteralKind::UnicodeShort);
    assert!(result.is_err());
}

