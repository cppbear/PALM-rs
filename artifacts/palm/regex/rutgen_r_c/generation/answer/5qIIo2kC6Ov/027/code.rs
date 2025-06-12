// Answer 0

#[test]
fn test_parse_with_comments_basic() {
    struct DummyParser {
        pos: Cell<Position>,
        comments: RefCell<Vec<ast::Comment>>,
        input: Vec<char>,
        index: usize,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                comments: RefCell::new(vec![]),
                input: pattern.chars().collect(),
                index: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.index]
            }
        }

        fn bump_space(&mut self) {
            while !self.is_eof() && self.char().is_whitespace() {
                self.index += 1;
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.index += 1;
            }
        }

        fn parse_set_class(&self) -> Result<ast::Class> {
            Ok(ast::Class::Bracketed(ast::ClassBracketed {
                span: Span { start: self.pos.get(), end: self.pos.get() },
                items: vec![]
            }))
        }

        fn pop_group_end(&self, concat: ast::Concat) -> Result<ast::Concat> {
            Ok(concat)
        }

        fn reset(&self) {
            self.pos.set(Position { offset: 0, line: 1, column: 1 });
            self.comments.borrow_mut().clear();
        }

        fn span(&self) -> Span {
            Span { start: self.pos.get(), end: self.pos.get() }
        }
    }

    let parser = DummyParser::new("[a-z]");
    let mut concat = ast::Concat { span: parser.span(), asts: vec![] };

    Assert!(!parser.is_eof());

    parser.bump_space();
    assert_eq!(parser.char(), '[');
    concat = parser.parse_set_class().unwrap();
    
    while !parser.is_eof() {
        parser.bump();
    }

    let group_end = parser.pop_group_end(concat).unwrap();
    
    let comments: Vec<ast::Comment> = vec![];
    parser.comments.borrow_mut().extend(comments);
    let with_comments = Ok(ast::WithComments {
        ast: Ast::Concat(group_end),
        comments: mem::replace(&mut *parser.comments.borrow_mut(), vec![]),
    });
    
    assert!(with_comments.is_ok());
}

#[test]
fn test_parse_with_comments_error_eof() {
    struct DummyParser {
        pos: Cell<Position>,
        comments: RefCell<Vec<ast::Comment>>,
        input: Vec<char>,
        index: usize,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                comments: RefCell::new(vec![]),
                input: pattern.chars().collect(),
                index: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.index]
            }
        }

        fn bump_space(&mut self) {
            while !self.is_eof() && self.char().is_whitespace() {
                self.index += 1;
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.index += 1;
            }
        }

        fn parse_set_class(&self) -> Result<ast::Class> {
            Ok(ast::Class::Bracketed(ast::ClassBracketed {
                span: Span { start: self.pos.get(), end: self.pos.get() },
                items: vec![]
            }))
        }

        fn pop_group_end(&self, concat: ast::Concat) -> Result<ast::Concat> {
            Ok(concat)
        }

        fn reset(&self) {
            self.pos.set(Position { offset: 0, line: 1, column: 1 });
            self.comments.borrow_mut().clear();
        }

        fn span(&self) -> Span {
            Span { start: self.pos.get(), end: self.pos.get() }
        }
    }

    let parser = DummyParser::new("[a-z");
    let mut concat = ast::Concat { span: parser.span(), asts: vec![] };

    parser.bump_space();
    assert_eq!(parser.char(), '[');
    concat = parser.parse_set_class().unwrap();
    
    while !parser.is_eof() {
        parser.bump();
    }

    let group_end_result = parser.pop_group_end(concat);
    assert!(group_end_result.is_err());
}

