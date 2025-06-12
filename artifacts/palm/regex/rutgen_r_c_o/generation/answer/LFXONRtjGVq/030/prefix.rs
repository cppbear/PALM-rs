// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_opening_brace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let concat = ast::Concat { span: span.clone(), asts: vec![] };
    
    let parser = ParserI {
        parser: Parser { /* initialize required parser fields */ },
        pattern: "a",
    };
    
    // Here self.char() is not '{', it is 'a' in this case.
    // Must call parse_counted_repetition with this setup
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let concat = ast::Concat { span: span.clone(), asts: vec![] };
    
    let parser = ParserI {
        parser: Parser { /* initialize required parser fields */ },
        pattern: "{1,2}",
    };

    // Must call parse_counted_repetition with an empty concat.asts
    let _ = parser.parse_counted_repetition(concat);
}

