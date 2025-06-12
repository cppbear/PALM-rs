// Answer 0

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_A() {
    let pattern = "ABC";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_B() {
    let pattern = "123";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_C() {
    let pattern = "9";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_D() {
    let pattern = "G";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_E() {
    let pattern = "H";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_F() {
    let pattern = "8";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

#[test]
#[should_panic]
fn test_parse_octal_char_not_in_range_G() {
    let pattern = "7";
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser { octal: true, pos: Cell::new(position) };
    let parser_i = ParserI { parser: &parser, pattern };
    parser_i.parse_octal();
}

