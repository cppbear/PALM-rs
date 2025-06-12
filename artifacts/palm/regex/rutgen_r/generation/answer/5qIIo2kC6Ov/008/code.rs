// Answer 0

#[test]
fn test_parse_with_comments_success() {
    struct TestParser {
        eof: bool,
        char: char,
        offset: usize,
        comments: Vec<String>,
    }

    impl TestParser {
        fn new() -> Self {
            Self { eof: false, char: '{', offset: 0, comments: vec![] }
        }

        fn bump_space(&mut self) {}
        fn is_eof(&self) -> bool { self.eof }
        fn char(&self) -> char { self.char }
        fn offset(&self) -> usize { self.offset }
        fn parser(&self) -> &ParserMock { &ParserMock { comments: self.comments.clone() } }

        fn push_group(&self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
        fn pop_group(&self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
        fn push_alternate(&self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
        fn parse_set_class(&self) -> Result<ast::Class> { Ok(ast::Class) }
        
        fn parse_uncounted_repetition(&self, concat: ast::Concat, _: ast::RepetitionKind) -> Result<ast::Concat> { 
            Ok(concat) 
        }

        fn parse_counted_repetition(&self, concat: ast::Concat) -> Result<ast::Concat> { 
            Err("Error parsing counted repetition".into()) 
        }

        fn parse_primitive(&self) -> Result<Primitive> { Ok(Primitive) }
        fn pop_group_end(&self, concat: ast::Concat) -> Result<ast::Ast> { Ok(ast::Ast) }
    }

    struct ParserMock {
        comments: Vec<String>,
    }

    impl ParserMock {
        fn reset(&mut self) {}
    }

    let parser = TestParser::new();
    
    let result = parser.parse_with_comments();
    
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_with_comments_panic_on_offset() {
    struct TestParser {
        offset: usize,
        eof: bool,
    }

    impl TestParser {
        fn new() -> Self {
            Self { offset: 1, eof: false }
        }

        fn bump_space(&mut self) {}
        fn is_eof(&self) -> bool { self.eof }
        fn char(&self) -> char { '{' }
        fn offset(&self) -> usize { self.offset }
        fn parser(&self) -> &ParserMock { &ParserMock { comments: vec![] } }

        fn push_group(&self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
        fn pop_group(&self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
        fn push_alternate(&self, concat: ast::Concat) -> Result<ast::Concat> { Ok(concat) }
        fn parse_set_class(&self) -> Result<ast::Class> { Ok(ast::Class) }
        
        fn parse_uncounted_repetition(&self, concat: ast::Concat, _: ast::RepetitionKind) -> Result<ast::Concat> { 
            Ok(concat) 
        }

        fn parse_counted_repetition(&self, concat: ast::Concat) -> Result<ast::Concat> { 
            Ok(concat) 
        }

        fn parse_primitive(&self) -> Result<Primitive> { Ok(Primitive) }
        fn pop_group_end(&self, concat: ast::Concat) -> Result<ast::Ast> { Ok(ast::Ast) }
    }

    struct ParserMock {
        comments: Vec<String>,
    }

    impl ParserMock {
        fn reset(&mut self) {}
    }

    let parser = TestParser::new();
    
    let _ = parser.parse_with_comments();
}

