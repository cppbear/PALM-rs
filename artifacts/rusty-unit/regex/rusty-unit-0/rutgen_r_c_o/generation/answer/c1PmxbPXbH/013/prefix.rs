// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let ast = Ast::Literal(ast::Literal { span });
    let concat = Concat { span, asts: vec![ast] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a?" };

    let result = parser.parse_uncounted_repetition(concat.clone(), RepetitionKind::ZeroOrOne);

    // The return value is not asserted, only the call is made.
}

#[test]
fn test_parse_uncounted_repetition_with_star() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position, position);
    let ast = Ast::Class(ast::Class { span });
    let concat = Concat { span, asts: vec![ast] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a*" };

    let result = parser.parse_uncounted_repetition(concat.clone(), RepetitionKind::ZeroOrMore);

    // The return value is not asserted, only the call is made.
}

#[test]
fn test_parse_uncounted_repetition_with_plus() {
    let position = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(position, position);
    let ast = Ast::Group(ast::Group { span });
    let concat = Concat { span, asts: vec![ast] };
    let parser = ParserI { parser: Parser { pos: Cell::new(position), capture_index: Cell::new(0), nest_limit: 10, octal: false, initial_ignore_whitespace: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a+" };

    let result = parser.parse_uncounted_repetition(concat.clone(), RepetitionKind::OneOrMore);

    // The return value is not asserted, only the call is made.
}

