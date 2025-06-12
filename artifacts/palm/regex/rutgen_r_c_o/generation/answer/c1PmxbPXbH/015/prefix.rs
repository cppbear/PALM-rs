// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let ast = Ast::Literal(Literal { span });
    let concat = ast::Concat { span, asts: vec![ast] };
    let parser = ParserI { parser: Parser {}, pattern: "a?" };
    
    parser.parse_uncounted_repetition(concat, RepetitionKind::ZeroOrOne).unwrap();
}

#[test]
fn test_parse_uncounted_repetition_with_asterisk() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let ast = Ast::Class(Class { span });
    let concat = ast::Concat { span, asts: vec![ast] };
    let parser = ParserI { parser: Parser {}, pattern: "a*" };
    
    parser.parse_uncounted_repetition(concat, RepetitionKind::ZeroOrMore).unwrap();
}

#[test]
fn test_parse_uncounted_repetition_with_plus() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let ast = Ast::Group(Group { span });
    let concat = ast::Concat { span, asts: vec![ast] };
    let parser = ParserI { parser: Parser {}, pattern: "a+" };
    
    // Expecting this to panic due to self.char() not being '+' before usage
    let result = std::panic::catch_unwind(|| {
        parser.parse_uncounted_repetition(concat, RepetitionKind::OneOrMore).unwrap();
    });
    assert!(result.is_err());
}

