// Answer 0

#[test]
fn test_parse_with_comments_simple() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a|b");
}

#[test]
fn test_parse_with_comments_grouping() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("(a|b)");
}

#[test]
fn test_parse_with_comments_repetition() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a*");
}

#[test]
fn test_parse_with_comments_range() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a{2,5}");
}

#[test]
fn test_parse_with_comments_class() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("[a-z]");
}

#[test]
fn test_parse_with_comments_nested_groups() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("((a|b)c)?");
}

#[test]
fn test_parse_with_comments_empty() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("");
}

#[test]
fn test_parse_with_comments_eof() {
    let parser = Parser::new();
    let result = parser.parse_with_comments("a(b|c)");
}

