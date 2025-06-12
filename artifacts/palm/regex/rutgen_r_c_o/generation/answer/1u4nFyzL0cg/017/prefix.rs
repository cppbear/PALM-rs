// Answer 0

#[test]
fn test_parse_set_class_simple() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[a-z]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_nested() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[[a-z]-[0-9]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_intersection() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[a-z&&[b-d]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_difference() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[a-z--[c-e]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_symmetric_difference() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[a-z~~[b-d]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_empty() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_nested_ascii_class() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[[:alpha:]]",
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_complex() {
    let parser = ParserI {
        parser: Parser::new(),
        pattern: "[a-zA-Z0-9&&[a-f]]",
    };
    parser.parse_set_class();
}

