// Answer 0

#[test]
fn test_parse_hex_digits_valid_x() {
    let parser = ParserI {
        parser: Parser {
            scratch: RefCell::new(String::new()),
            ..Default::default()
        },
        pattern: "\\x61",
    };
    parser.parse_hex_digits(ast::HexLiteralKind::X);
}

#[test]
fn test_parse_hex_digits_valid_unicode_short() {
    let parser = ParserI {
        parser: Parser {
            scratch: RefCell::new(String::new()),
            ..Default::default()
        },
        pattern: "\\u0061",
    };
    parser.parse_hex_digits(ast::HexLiteralKind::UnicodeShort);
}

#[test]
fn test_parse_hex_digits_valid_unicode_long() {
    let parser = ParserI {
        parser: Parser {
            scratch: RefCell::new(String::new()),
            ..Default::default()
        },
        pattern: "\\U00000061",
    };
    parser.parse_hex_digits(ast::HexLiteralKind::UnicodeLong);
}

#[test]
#[should_panic]
fn test_parse_hex_digits_invalid_character() {
    let parser = ParserI {
        parser: Parser {
            scratch: RefCell::new(String::new()),
            ..Default::default()
        },
        pattern: "\\xG1", // invalid hex character 'G'
    };
    parser.parse_hex_digits(ast::HexLiteralKind::X);
}

#[test]
#[should_panic]
fn test_parse_hex_digits_exceeds_digits() {
    let parser = ParserI {
        parser: Parser {
            scratch: RefCell::new(String::new()),
            ..Default::default()
        },
        pattern: "\\u0061\\u007A", // more characters than expected for UnicodeShort
    };
    parser.parse_hex_digits(ast::HexLiteralKind::UnicodeShort);
}

