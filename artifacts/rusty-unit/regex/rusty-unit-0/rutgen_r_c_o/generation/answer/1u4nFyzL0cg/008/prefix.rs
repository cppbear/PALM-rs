// Answer 0

#[test]
fn test_parse_set_class_basic_range() {
    let pattern = "[a-z]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_empty_range() {
    let pattern = "[--]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_difference() {
    let pattern = "[a-z--]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_intersection() {
    let pattern = "[a-z&&[b-z]]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_prefix_empty_range() {
    let pattern = "[--a-z]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_empty_range_and_intersection() {
    let pattern = "[a-z&&-]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_space_with_boundary() {
    let pattern = "[ -]";
    let parser = ParserI {
        parser: Parser::new(),
        pattern: pattern,
    };
    parser.parse_set_class();
}

