// Answer 0

#[test]
fn test_parse_counted_repetition_repetition_missing() {
    struct TestParser<'a> {
        char: char,
        pos: Position,
        concat: ast::Concat,
    }

    impl<'a> Borrow<Parser> for TestParser<'a> {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            &Parser {} 
        }
    }

    let pos = Position { offset: 0, line: 1, column: 1 };
    let concat = ast::Concat { span: Span::new(pos, pos), asts: vec![] };
    let parser = TestParser { char: '{', pos, concat };

    let result = parser.parse_counted_repetition(parser.concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_repetition_count_unclosed() {
    struct TestParser<'a> {
        char: char,
        pos: Position,
        concat: ast::Concat,
    }

    impl<'a> Borrow<Parser> for TestParser<'a> {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            &Parser {} 
        }
    }

    let pos = Position { offset: 0, line: 1, column: 1 };
    let ast = Ast::Literal(ast::Literal { span: Span::new(pos, pos) });
    let concat = ast::Concat { span: Span::new(pos, pos), asts: vec![ast] };
    let parser = TestParser { char: '{', pos, concat };

    let result = parser.parse_counted_repetition(parser.concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_valid_case() {
    struct TestParser {
        char: char,
        pos: Position,
        concat: ast::Concat,
        digits: u32,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            // dummy implementation
            &Parser {}
        }
    }

    let pos_start = Position { offset: 0, line: 1, column: 1 };
    let pos_end = Position { offset: 1, line: 1, column: 2 };
    let ast = Ast::Literal(ast::Literal { span: Span::new(pos_start, pos_end) });
    let concat = ast::Concat { span: Span::new(pos_start, pos_end), asts: vec![ast] };

    let parser = TestParser { char: '{', pos: pos_start, concat, digits: 3 };

    let result = parser.parse_counted_repetition(parser.concat);
    assert!(result.is_ok());
}

