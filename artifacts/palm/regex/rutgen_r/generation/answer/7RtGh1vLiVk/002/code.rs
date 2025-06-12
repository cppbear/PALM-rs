// Answer 0

#[test]
fn test_push_alternate_with_new_alternation() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
        alternations: Vec<ast::Concat>,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                pos: 0,
                input: input.chars().collect(),
                alternations: vec![],
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn push_or_add_alternation(&mut self, concat: ast::Concat) {
            if let Some(last) = self.alternations.last_mut() {
                last.ast.push(concat);
            } else {
                self.alternations.push(concat);
            }
        }

        fn span(&self) -> ast::Span {
            ast::Span { start: self.pos, end: self.pos }
        }
    }

    let mut parser = Parser::new("|a|b");
    let concat = ast::Concat {
        span: ast::Span { start: 0, end: 0 },
        asts: vec![ast::Ast::Char('a')],
    };

    let result = parser.push_alternate(concat).unwrap();

    assert_eq!(result.span.start, 1);
    assert_eq!(result.span.end, 1);
    assert!(result.asts.is_empty());

    assert_eq!(parser.pos(), 1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_push_alternate_should_panic_on_non_pipe() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }
    }

    let mut parser = Parser::new("a");

    let concat = ast::Concat {
        span: ast::Span { start: 0, end: 0 },
        asts: vec![],
    };

    parser.push_alternate(concat).unwrap();
}

