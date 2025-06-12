// Answer 0

#[test]
fn test_parse_escape_valid_u() {
    let pattern = "\\U";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_d() {
    let pattern = "\\d";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_x() {
    let pattern = "\\x";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_w() {
    let pattern = "\\w";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_W() {
    let pattern = "\\W";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_u_upper() {
    let pattern = "\\U";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_P() {
    let pattern = "\\P";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_s() {
    let pattern = "\\s";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_S() {
    let pattern = "\\S";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_p() {
    let pattern = "\\p";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

#[test]
fn test_parse_escape_valid_a() {
    let pattern = "\\a";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_escape();
}

