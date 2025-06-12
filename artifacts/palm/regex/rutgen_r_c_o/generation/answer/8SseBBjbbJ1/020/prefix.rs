// Answer 0

#[test]
fn test_parse_hex_with_x_prefix_and_brace() {
    let input = r"\x{FF}";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: input,
    };
    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_u_prefix_and_brace() {
    let input = r"\u{FFFF}";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: input,
    };
    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_x_prefix_no_brace() {
    let input = r"\xFF";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: input,
    };
    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_u_prefix_no_brace() {
    let input = r"\uFF";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: input,
    };
    parser.parse_hex();
}

#[test]
fn test_parse_hex_with_U_prefix_no_brace() {
    let input = r"\UFFFF";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: input,
    };
    parser.parse_hex();
}

