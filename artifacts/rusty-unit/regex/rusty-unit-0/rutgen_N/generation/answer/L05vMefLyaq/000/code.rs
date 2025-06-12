// Answer 0

#[derive(Debug)]
struct MockParser {
    stack_class: std::cell::RefCell<Vec<ClassState>>,
}

impl MockParser {
    fn new() -> Self {
        MockParser {
            stack_class: std::cell::RefCell::new(Vec::new()),
        }
    }
}

#[derive(Debug)]
struct Set {
    span: usize,
}

#[derive(Debug)]
enum ClassState {
    Open { set: Set },
    Closed,
}

#[derive(Debug)]
struct MockAst {
   parser: MockParser,
}

impl MockAst {
    fn new() -> Self {
        MockAst {
            parser: MockParser::new(),
        }
    }

    fn parser(&self) -> &MockParser {
        &self.parser
    }

    fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
        ast::Error { message: "Unclosed character class".into() }
    }

    fn unclosed_class_error(&self) -> ast::Error {
        for state in self.parser().stack_class.borrow().iter().rev() {
            match *state {
                ClassState::Open { ref set } => {
                    return self.error(set.span, ast::ErrorKind::ClassUnclosed);
                }
                _ => {}
            }
        }
        panic!("no open character class found")
    }
}

mod ast {
    #[derive(Debug)]
    pub struct Error {
        pub message: String,
    }

    #[derive(Debug)]
    pub enum ErrorKind {
        ClassUnclosed,
    }
}

#[test]
fn test_unclosed_class_error_with_open_class() {
    let mut mock_ast = MockAst::new();
    mock_ast.parser.stack_class.borrow_mut().push(ClassState::Open { set: Set { span: 5 } });
    
    let error = mock_ast.unclosed_class_error();
    assert_eq!(error.message, "Unclosed character class");
}

#[test]
#[should_panic(expected = "no open character class found")]
fn test_unclosed_class_error_with_no_open_class() {
    let mock_ast = MockAst::new();
    let _error = mock_ast.unclosed_class_error(); // Should panic here
}

