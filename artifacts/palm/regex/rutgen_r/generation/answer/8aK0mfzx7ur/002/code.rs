// Answer 0

#[test]
fn test_pop_group_end_no_state() {
    struct Parser {
        stack_group: std::cell::RefCell<Vec<GroupState>>,
        position: usize,
    }

    impl Parser {
        fn new() -> Self {
            Parser {
                stack_group: std::cell::RefCell::new(Vec::new()),
                position: 0,
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new()
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    struct Concat {
        span: Span,
    }

    impl Concat {
        fn new() -> Self {
            Concat {
                span: Span::new(0, 0),
            }
        }

        fn into_ast(self) -> Ast {
            Ast::Concat
        }
    }

    let concat = Concat::new();
    let parser = Parser::new();

    let result = parser.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
fn test_pop_group_end_with_alternation() {
    struct Parser {
        stack_group: std::cell::RefCell<Vec<GroupState>>,
        position: usize,
    }

    impl Parser {
        fn new() -> Self {
            Parser {
                stack_group: std::cell::RefCell::new(vec![GroupState::Alternation(Alternation::new())]),
                position: 3,
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new()
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    struct Concat {
        span: Span,
    }

    impl Concat {
        fn new() -> Self {
            Concat {
                span: Span::new(0, 0),
            }
        }

        fn into_ast(self) -> Ast {
            Ast::Concat
        }
    }

    let concat = Concat::new();
    let parser = Parser::new();

    let result = parser.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "GroupUnclosed")]
fn test_pop_group_end_with_unclosed_group() {
    struct Parser {
        stack_group: std::cell::RefCell<Vec<GroupState>>,
        position: usize,
    }

    impl Parser {
        fn new() -> Self {
            Parser {
                stack_group: std::cell::RefCell::new(vec![GroupState::Group { group: Group::new() }]),
                position: 3,
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new()
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    struct Concat {
        span: Span,
    }

    impl Concat {
        fn new() -> Self {
            Concat {
                span: Span::new(0, 0),
            }
        }

        fn into_ast(self) -> Ast {
            Ast::Concat
        }
    }

    let concat = Concat::new();
    let parser = Parser::new();

    let _ = parser.pop_group_end(concat);
}

#[test]
fn test_pop_group_end_multiple_alternations() {
    struct Parser {
        stack_group: std::cell::RefCell<Vec<GroupState>>,
        position: usize,
    }

    impl Parser {
        fn new() -> Self {
            Parser {
                stack_group: std::cell::RefCell::new(vec![GroupState::Alternation(Alternation::new()), GroupState::Alternation(Alternation::new())]),
                position: 3,
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new()
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    struct Concat {
        span: Span,
    }

    impl Concat {
        fn new() -> Self {
            Concat {
                span: Span::new(0, 0),
            }
        }

        fn into_ast(self) -> Ast {
            Ast::Concat
        }
    }

    let concat = Concat::new();
    let parser = Parser::new();

    let result = parser.pop_group_end(concat);
    assert!(result.is_ok());
}

