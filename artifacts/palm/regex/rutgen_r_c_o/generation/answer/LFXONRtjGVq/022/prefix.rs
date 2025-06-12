// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_open_brace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let concat = Concat { span: Span::new(position, position), asts: vec![Ast::Literal(ast::Literal::empty(position))] };

    let parser = ParserI {
        parser: Parser { /* Initialize fields as necessary */ },
        pattern: "some pattern with {",
    };

    parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_no_ast_to_pop() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let concat = Concat { span: Span::new(position, position), asts: vec![] };

    let parser = ParserI {
        parser: Parser { /* Initialize fields as necessary */ },
        pattern: "{3,5} more text",
    };

    parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_bump_and_bump_space_false() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let concat = Concat { span: Span::new(position, position), asts: vec![Ast::Literal(ast::Literal::empty(position))] };

    let parser = ParserI {
        parser: Parser { /* Initialize fields as necessary */ },
        pattern: "{3,5} more text",
    };

    parser.bump_and_bump_space = || false;

    parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_is_eof_true_after_parse_decimal() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let concat = Concat { span: Span::new(position, position), asts: vec![Ast::Literal(ast::Literal::empty(position))] };

    let parser = ParserI {
        parser: Parser { /* Initialize fields as necessary */ },
        pattern: "{3,5} more text",
    };

    parser.is_eof = || true;

    parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_char_is_comma() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let concat = Concat { span: Span::new(position, position), asts: vec![Ast::Literal(ast::Literal::empty(position))] };

    let parser = ParserI {
        parser: Parser { /* Initialize fields as necessary */ },
        pattern: "{3,5} more text",
    };

    parser.char = || ',';

    parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_char_not_closing_brace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let concat = Concat { span: Span::new(position, position), asts: vec![Ast::Literal(ast::Literal::empty(position))] };

    let parser = ParserI {
        parser: Parser { /* Initialize fields as necessary */ },
        pattern: "{3,5} more text",
    };

    parser.char = || 'a';  // Any character other than '}'

    parser.parse_counted_repetition(concat);
}

