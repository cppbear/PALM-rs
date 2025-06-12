// Answer 0

#[test]
fn test_parse_with_comments_valid_input() {
    let parser = {
        struct SimpleParser {
            chars: Vec<char>,
            pos: usize,
            comments: Vec<ast::Comment>,
        }
        
        impl SimpleParser {
            fn new(chars: Vec<char>) -> Self {
                Self { chars, pos: 0, comments: vec![] }
            }

            fn bump_space(&mut self) {}
            fn is_eof(&self) -> bool { self.pos >= self.chars.len() }
            fn char(&self) -> char { self.chars[self.pos] }
            fn push_group(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn pop_group(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn push_alternate(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn parse_set_class(&self) -> Result<ast::Class> { Ok(ast::Class::Bracketed(ast::ClassBracketed { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 2, line: 1, column: 3 } }, items: vec![] })) }
            fn pop_group_end(&self, concat: ast::Concat) -> Result<Ast> { Ok(Ast::Concat(concat)) }
            fn parse_primitive(&self) -> Result<Primitive> {
                Ok(Primitive::Literal(ast::Literal { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } }, kind: ast::LiteralKind::Verbatim, c: '+' }))
            }
        }

        SimpleParser::new(vec!['+', '(', 'a', ')'])
    };

    let parser_i = ParserI { parser: &parser, pattern: "+" };
    
    let result = parser_i.parse_with_comments();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "parser can only be used once")]
fn test_parse_with_comments_parser_once() {
    let parser = {
        struct SimpleParser {
            chars: Vec<char>,
            pos: usize,
            comments: Vec<ast::Comment>,
        }
        
        impl SimpleParser {
            fn new(chars: Vec<char>) -> Self {
                Self { chars, pos: 0, comments: vec![] }
            }

            fn bump_space(&mut self) {}
            fn is_eof(&self) -> bool { self.pos >= self.chars.len() }
            fn char(&self) -> char { self.chars[self.pos] }
            fn push_group(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn pop_group(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn push_alternate(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn parse_set_class(&self) -> Result<ast::Class> { Ok(ast::Class::Bracketed(ast::ClassBracketed { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 2, line: 1, column: 3 } }, items: vec![] })) }
            fn pop_group_end(&self, concat: ast::Concat) -> Result<Ast> { Ok(Ast::Concat(concat)) }
            fn parse_primitive(&self) -> Result<Primitive> {
                Ok(Primitive::Literal(ast::Literal { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } }, kind: ast::LiteralKind::Verbatim, c: '+' }))
            }
        }

        SimpleParser::new(vec!['+', '(', 'a', ')'])
    };

    let parser_i = ParserI { parser: &parser, pattern: "+" };
    let _ = parser_i.parse_with_comments(); // First call, should not panic

    // Second call, should panic
    let _ = parser_i.parse_with_comments(); 
}

#[test]
fn test_parse_with_comments_valid_repetition() {
    let parser = {
        struct SimpleParser {
            chars: Vec<char>,
            pos: usize,
            comments: Vec<ast::Comment>,
        }
        
        impl SimpleParser {
            fn new(chars: Vec<char>) -> Self {
                Self { chars, pos: 0, comments: vec![] }
            }

            fn bump_space(&mut self) {}
            fn is_eof(&self) -> bool { self.pos >= self.chars.len() }
            fn char(&self) -> char { self.chars[self.pos] }
            fn push_group(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn pop_group(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn push_alternate(&mut self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
            fn parse_set_class(&self) -> Result<ast::Class> { Ok(ast::Class::Bracketed(ast::ClassBracketed { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 2, line: 1, column: 3 } }, items: vec![] })) }
            fn pop_group_end(&self, concat: ast::Concat) -> Result<Ast> { Ok(Ast::Concat(concat)) }
            fn parse_primitive(&self) -> Result<Primitive> {
                Ok(Primitive::Literal(ast::Literal { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } }, kind: ast::LiteralKind::Verbatim, c: '+' }))
            }
            fn parse_uncounted_repetition(&self, concat: ast::Concat, kind: ast::RepetitionKind) -> Result<ast::Concat> {
                if kind == ast::RepetitionKind::OneOrMore { return Err(ast::Error { kind: ast::ErrorKind::RepetitionMissing, pattern: "error".to_string(), span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } } }) }
                Ok(concat)
            }
        }

        SimpleParser::new(vec!['+', '(', 'a', ')'])
    };

    let parser_i = ParserI { parser: &parser, pattern: "+" };
    
    let result = parser_i.parse_with_comments();
    assert!(result.is_err());
}

