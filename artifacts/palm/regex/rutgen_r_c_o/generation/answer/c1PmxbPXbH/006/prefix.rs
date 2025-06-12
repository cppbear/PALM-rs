// Answer 0

#[test]
fn test_parse_uncounted_repetition_empty_concat() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(pos, pos);
    let concat = ast::Concat { span: span.clone(), asts: Vec::new() };
    let parser = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "abc" };

    let _result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
}

#[test]
fn test_parse_uncounted_repetition_invalid_char() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(pos, pos);
    let concat = ast::Concat { span: span.clone(), asts: Vec::new() };
    let parser = ParserI { parser: Parser { pos: Cell::new(pos), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "abc" };

    parser.parser.pos.set(pos); // This is where the char is not '?' or '*'
    // Simulate that the character is something invalid like 'a'
    let _result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
}

