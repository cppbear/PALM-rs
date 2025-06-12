// Answer 0

#[test]
fn test_pop_group_with_valid_group() {
    struct MockParser {
        stack_group: Vec<Group>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            ')'
        }

        fn span_char(&self) -> usize {
            self.position
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) {
            self.position += 1;
        }
    }

    struct MockAstConcat {
        asts: Vec<Ast>,
        span: Span,
    }

    impl MockAstConcat {
        fn new() -> Self {
            Self {
                asts: Vec::new(),
                span: Span { end: 0 },
            }
        }
    }

    struct Group {
        concat: MockAstConcat,
        group: GroupData,
        ignore_whitespace: bool,
    }

    struct GroupData {
        ast: Box<Ast>,
        span: Span,
    }

    struct Ast;

    struct Span {
        end: usize,
    }

    struct Result;

    let mut parser = MockParser {
        stack_group: vec![
            Group {
                concat: MockAstConcat::new(),
                group: GroupData { ast: Box::new(Ast), span: Span { end: 0 }},
                ignore_whitespace: false,
            },
        ],
        position: 0,
    };

    let concat = MockAstConcat::new();
    let result = parser.pop_group(concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_pop_group_with_unopened_group() {
    struct MockParser {
        stack_group: Vec<Group>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            ')'
        }

        fn span_char(&self) -> usize {
            self.position
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) {
            self.position += 1;
        }
    }

    struct MockAstConcat {
        asts: Vec<Ast>,
        span: Span,
    }

    impl MockAstConcat {
        fn new() -> Self {
            Self {
                asts: Vec::new(),
                span: Span { end: 0 },
            }
        }
    }

    struct Group;

    let mut parser = MockParser {
        stack_group: Vec::new(),
        position: 0,
    };

    let concat = MockAstConcat::new();
    let _ = parser.pop_group(concat);
}

