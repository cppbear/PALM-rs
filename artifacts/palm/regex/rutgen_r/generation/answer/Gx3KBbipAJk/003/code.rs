// Answer 0

#[test]
fn test_pop_group_success_with_group() {
    struct MockParser {
        stack_group: std::cell::RefCell<Vec<Group>>,
        position: usize,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_group: std::cell::RefCell::new(vec![Group {
                    concat: ast::Concat { asts: vec![] },
                    group: ast::Group { ast: Box::new(ast::Empty) },
                    ignore_whitespace: false,
                }]),
                position: 0,
            }
        }

        fn char(&self) -> char {
            ')'
        }

        fn pos(&mut self) -> usize {
            self.position
        }

        fn bump(&mut self) {
            self.position += 1;
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new();
    let group_concat = ast::Concat { asts: vec![ast::Empty] };
    
    let result = parser.pop_group(group_concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_pop_group_no_group_unopened() {
    struct MockParser {
        stack_group: std::cell::RefCell<Vec<Group>>,
        position: usize,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_group: std::cell::RefCell::new(vec![]),
                position: 0,
            }
        }

        fn char(&self) -> char {
            ')'
        }

        fn pos(&mut self) -> usize {
            self.position
        }

        fn bump(&mut self) {
            self.position += 1;
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new();
    let group_concat = ast::Concat { asts: vec![ast::Empty] };
    
    parser.pop_group(group_concat).unwrap();
}

#[test]
fn test_pop_group_success_with_alternation() {
    struct MockParser {
        stack_group: std::cell::RefCell<Vec<Alternation>>,
        position: usize,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_group: std::cell::RefCell::new(vec![Alternation {
                    asts: vec![ast::Empty],
                }, Group {
                    concat: ast::Concat { asts: vec![] },
                    group: ast::Group { ast: Box::new(ast::Empty) },
                    ignore_whitespace: false,
                }]),
                position: 0,
            }
        }

        fn char(&self) -> char {
            ')'
        }

        fn pos(&mut self) -> usize {
            self.position
        }

        fn bump(&mut self) {
            self.position += 1;
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = MockParser::new();
    let group_concat = ast::Concat { asts: vec![ast::Empty] };
    
    let result = parser.pop_group(group_concat);
    assert!(result.is_ok());
}

