// Answer 0

#[test]
fn test_pop_group_success() {
    struct MockParser {
        stack_group: RefCell<Vec<GroupState>>,
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_group: RefCell::new(vec![]),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                ignore_whitespace: Cell::new(false),
                pattern: String::from("abc(123)"),
            }
        }

        fn char(&self) -> char {
            ')'
        }

        fn bump(&self) {
            self.pos.set(Position {
                offset: self.pos.get().offset + 1,
                line: self.pos.get().line,
                column: self.pos.get().column + 1,
            });
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn span(&self) -> Span {
            Span { start: self.pos(), end: self.pos() }
        }
    }

    impl<'s> ParserI<'s, MockParser> {
        fn pop_group(&self, mut group_concat: ast::Concat) -> Result<ast::Concat> {
            use self::GroupState::*;

            assert_eq!(self.char(), ')');
            let mut stack = self.parser().stack_group.borrow_mut();
            let (mut prior_concat, mut group, ignore_whitespace, alt) =
                match stack.pop() {
                    Some(Group { concat, group, ignore_whitespace }) => {
                        (concat, group, ignore_whitespace, None)
                    }
                    Some(Alternation(alt)) => {
                        match stack.pop() {
                            Some(Group { concat, group, ignore_whitespace }) => {
                                (concat, group, ignore_whitespace, Some(alt))
                            }
                            None | Some(Alternation(_)) => {
                                return Err(self.error(self.span(), ast::ErrorKind::GroupUnopened));
                            }
                        }
                    }
                    None => {
                        return Err(self.error(self.span(), ast::ErrorKind::GroupUnopened));
                    }
                };
            self.parser().ignore_whitespace.set(ignore_whitespace);
            group_concat.span.end = self.pos();
            self.bump();
            group.span.end = self.pos();
            match alt {
                Some(mut alt) => {
                    alt.span.end = group_concat.span.end;
                    alt.asts.push(group_concat.into_ast());
                    group.ast = Box::new(alt.into_ast());
                }
                None => {
                    group.ast = Box::new(group_concat.into_ast());
                }
            }
            prior_concat.asts.push(Ast::Group(group));
            Ok(prior_concat)
        }
    }

    let mut parser = MockParser::new();
    let group_concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 3, line: 1, column: 4 }},
        asts: vec![],
    };

    parser.stack_group.borrow_mut().push(GroupState::Group { concat: group_concat.clone(), group: ast::Group { span: Span::new(parser.pos(), parser.pos()), kind: ast::GroupKind::Regular, ast: Box::new(ast::Empty(group_concat.span)) }, ignore_whitespace: false });
    
    let result = parser.pop_group(group_concat).unwrap();
    
    assert!(matches!(result.asts.last().unwrap(), Ast::Group(_)));
}

#[test]
#[should_panic(expected = "GroupUnopened")]
fn test_pop_group_unopened() {
    struct MockParser {
        stack_group: RefCell<Vec<GroupState>>,
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_group: RefCell::new(vec![]),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                ignore_whitespace: Cell::new(false),
                pattern: String::from("abc(123)"),
            }
        }

        fn char(&self) -> char {
            ')'
        }

        fn bump(&self) {
            // Simulate just moving past closing parenthesis.
            self.pos.set(Position {
                offset: 1,
                line: 1,
                column: 1,
            });
        }
    
        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn span(&self) -> Span {
            Span { start: self.pos(), end: self.pos() }
        }
    }

    impl<'s> ParserI<'s, MockParser> {
        fn pop_group(&self, mut group_concat: ast::Concat) -> Result<ast::Concat> {
            use self::GroupState::*;

            assert_eq!(self.char(), ')');
            let mut stack = self.parser().stack_group.borrow_mut();
            let (mut prior_concat, mut group, ignore_whitespace, alt) =
                match stack.pop() {
                    Some(Group { concat, group, ignore_whitespace }) => {
                        (concat, group, ignore_whitespace, None)
                    }
                    Some(Alternation(alt)) => {
                        match stack.pop() {
                            Some(Group { concat, group, ignore_whitespace }) => {
                                (concat, group, ignore_whitespace, Some(alt))
                            }
                            None | Some(Alternation(_)) => {
                                return Err(self.error(self.span(), ast::ErrorKind::GroupUnopened));
                            }
                        }
                    }
                    None => {
                        return Err(self.error(self.span(), ast::ErrorKind::GroupUnopened));
                    }
                };
            self.parser().ignore_whitespace.set(ignore_whitespace);
            group_concat.span.end = self.pos();
            self.bump();
            group.span.end = self.pos();
            match alt {
                Some(mut alt) => {
                    alt.span.end = group_concat.span.end;
                    alt.asts.push(group_concat.into_ast());
                    group.ast = Box::new(alt.into_ast());
                }
                None => {
                    group.ast = Box::new(group_concat.into_ast());
                }
            }
            prior_concat.asts.push(Ast::Group(group));
            Ok(prior_concat)
        }
    }

    let mut parser = MockParser::new();
    let group_concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 3, line: 1, column: 4 }},
        asts: vec![],
    };

    // Not pushing any group states onto the stack, expecting to panic.
    parser.pop_group(group_concat).unwrap();
}

