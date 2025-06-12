// Answer 0

#[test]
fn test_push_alternate_with_single_alternation() {
    #[derive(Debug)]
    struct MockParser {
        pos: Position,
        stack_group: RefCell<Vec<Either<ast::Alternation, ast::Concat>>>,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                stack_group: RefCell::new(vec![]),
            }
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
            self.pos.column += 1;
        }

        fn char(&self) -> char {
            '|'
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn push_or_add_alternation(&self, concat: ast::Concat) {
            let mut stack = self.stack_group.borrow_mut();
            if let Some(Either::Left(ref mut alts)) = stack.last_mut() {
                alts.asts.push(concat);
            } else {
                stack.push(Either::Left(ast::Alternation {
                    span: Span { start: self.pos(), end: self.pos() },
                    asts: vec![concat],
                }));
            }
        }
    }

    let parser = MockParser::new();
    let concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 }},
        asts: vec![],
    };

    let result = parser.push_alternate(concat).unwrap();
    assert_eq!(result.asts.len(), 0);
}

#[test]
fn test_push_alternate_with_no_alternation() {
    #[derive(Debug)]
    struct MockParser {
        pos: Position,
        stack_group: RefCell<Vec<Either<ast::Alternation, ast::Concat>>>,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                stack_group: RefCell::new(vec![]),
            }
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
            self.pos.column += 1;
        }

        fn char(&self) -> char {
            '|'
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn push_or_add_alternation(&self, concat: ast::Concat) {
            let mut stack = self.stack_group.borrow_mut();
            if let Some(Either::Left(ref mut alts)) = stack.last_mut() {
                alts.asts.push(concat);
            } else {
                stack.push(Either::Left(ast::Alternation {
                    span: Span { start: self.pos(), end: self.pos() },
                    asts: vec![concat],
                }));
            }
        }
    }

    let parser = MockParser::new();
    let concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 }},
        asts: vec![],
    };

    let result = parser.push_alternate(concat).unwrap();
    assert!(result.asts.is_empty());
}

