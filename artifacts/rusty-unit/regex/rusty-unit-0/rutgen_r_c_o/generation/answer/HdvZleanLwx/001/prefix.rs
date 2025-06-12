// Answer 0

#[test]
fn test_parse_octal_3_digit_valid() {
    let parser = Parser { /* initialize with octal = true */ };
    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 3, line: 1, column: 4 });
    let literal = ast::Literal {
        span,
        kind: LiteralKind::Octal,
        c: '0',
    };
    let parser_i = ParserI { parser: &parser, pattern: "000" };
    parser_i.parse_octal();
}

#[test]
fn test_parse_octal_3_digit_max() {
    let parser = Parser { /* initialize with octal = true */ };
    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 3, line: 1, column: 4 });
    let literal = ast::Literal {
        span,
        kind: LiteralKind::Octal,
        c: '7',
    };
    let parser_i = ParserI { parser: &parser, pattern: "777" };
    parser_i.parse_octal();
}

#[test]
fn test_parse_octal_1_digit() {
    let parser = Parser { /* initialize with octal = true */ };
    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 });
    let literal = ast::Literal {
        span,
        kind: LiteralKind::Octal,
        c: '2',
    };
    let parser_i = ParserI { parser: &parser, pattern: "2" };
    parser_i.parse_octal();
}

#[test]
fn test_parse_octal_2_digit() {
    let parser = Parser { /* initialize with octal = true */ };
    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 });
    let literal = ast::Literal {
        span,
        kind: LiteralKind::Octal,
        c: '5',
    };
    let parser_i = ParserI { parser: &parser, pattern: "25" };
    parser_i.parse_octal();
}

