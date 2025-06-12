// Answer 0

#[test]
fn test_char_at_valid_start() {
    let parser = ParserI::new(Parser::default(), "hello");
    let result = parser.char_at(0);
}

#[test]
fn test_char_at_valid_middle() {
    let parser = ParserI::new(Parser::default(), "hello");
    let result = parser.char_at(2);
}

#[test]
fn test_char_at_valid_end() {
    let parser = ParserI::new(Parser::default(), "hello");
    let result = parser.char_at(4);
}

#[should_panic(expected = "expected char at offset 5")]
#[test]
fn test_char_at_panic_over_index() {
    let parser = ParserI::new(Parser::default(), "hello");
    let result = parser.char_at(5);
}

#[should_panic(expected = "expected char at offset 0")]
#[test]
fn test_char_at_panic_empty_string() {
    let parser = ParserI::new(Parser::default(), "");
    let result = parser.char_at(0);
}

