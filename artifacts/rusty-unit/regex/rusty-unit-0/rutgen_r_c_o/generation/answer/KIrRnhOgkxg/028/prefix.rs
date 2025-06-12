// Answer 0

#[test]
fn test_maybe_parse_ascii_class_no_colon_after_opening() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:lower:]]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_invalid_class_name() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:loower:]]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_with_extra_characters() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:lower:]A]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_negated() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:^upper:]]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_valid_name() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:alpha:]]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_without_closing_bracket() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:digit]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_empty_input() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_end_of_file() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:space:]",
    };
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_bump_false() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser { pos: Cell::new(position), ..Default::default() },
        pattern: "[[:ws:]]",
    };
    parser.maybe_parse_ascii_class();
}

