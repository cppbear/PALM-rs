// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "<abc>";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { parser: Parser {}, pattern: pattern };
    parser.pos.set(start_pos);
    parser.parse_capture_name(0);
}

#[test]
#[should_panic]
fn test_parse_capture_name_empty_name() {
    let pattern = "<>";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { parser: Parser {}, pattern: pattern };
    parser.pos.set(start_pos);
    parser.parse_capture_name(0);
}

#[test]
#[should_panic]
fn test_parse_capture_name_invalid_first_char() {
    let pattern = "<1abc>";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { parser: Parser {}, pattern: pattern };
    parser.pos.set(start_pos);
    parser.parse_capture_name(1);
}

#[test]
#[should_panic]
fn test_parse_capture_name_unexpected_eof() {
    let pattern = "<abc"; // missing '>'
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { parser: Parser {}, pattern: pattern };
    parser.pos.set(start_pos);
    parser.parse_capture_name(2);
}

#[test]
fn test_parse_capture_name_name_length_max() {
    let pattern = "<abcdefghij>"; // name length is 10
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { parser: Parser {}, pattern: pattern };
    parser.pos.set(start_pos);
    parser.parse_capture_name(3);
}

#[test]
#[should_panic]
fn test_parse_capture_name_invalid_char() {
    let pattern = "<abc$>";
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI { parser: Parser {}, pattern: pattern };
    parser.pos.set(start_pos);
    parser.parse_capture_name(4);
}

