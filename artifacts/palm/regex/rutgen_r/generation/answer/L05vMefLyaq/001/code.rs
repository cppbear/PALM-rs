// Answer 0

#[derive(Debug)]
struct Set {
    span: usize,
}

#[derive(Debug)]
enum ClassState {
    Open { set: Set },
    Closed,
}

struct Parser {
    stack_class: Vec<ClassState>,
}

impl Parser {
    fn borrow(&self) -> &Vec<ClassState> {
        &self.stack_class
    }
}

struct Context {
    parser: Parser,
}

impl Context {
    fn parser(&self) -> &Parser {
        &self.parser
    }

    fn error(&self, span: usize, kind: ast::ErrorKind) -> ast::Error {
        ast::Error { span, kind }
    }

    fn unclosed_class_error(&self) -> ast::Error {
        for state in self.parser().borrow().iter().rev() {
            match *state {
                ClassState::Open { ref set, .. } => {
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
    pub enum ErrorKind {
        ClassUnclosed,
    }

    #[derive(Debug)]
    pub struct Error {
        pub span: usize,
        pub kind: ErrorKind,
    }
}

#[test]
#[should_panic(expected = "no open character class found")]
fn test_unclosed_class_error_no_open_class() {
    let context = Context {
        parser: Parser {
            stack_class: vec![ClassState::Closed],
        },
    };
    context.unclosed_class_error();
}

#[test]
fn test_unclosed_class_error_with_open_class() {
    let set = Set { span: 5 };
    let context = Context {
        parser: Parser {
            stack_class: vec![
                ClassState::Closed,
                ClassState::Open { set },
            ],
        },
    };
    let error = context.unclosed_class_error();
    assert_eq!(error.span, 5);
    assert_eq!(error.kind, ast::ErrorKind::ClassUnclosed);
}

