// Answer 0

#[derive(Debug)]
struct MockParser {
    stack_class: std::cell::RefCell<Vec<ClassState>>,
}

#[derive(Debug)]
enum ClassState {
    Open { set: Span },
    Closed,
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Error {
    span: Span,
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    ClassUnclosed,
}

impl MockParser {
    fn new() -> Self {
        MockParser {
            stack_class: std::cell::RefCell::new(vec![]),
        }
    }

    fn stack_class(&self) -> std::cell::RefMut<'_, Vec<ClassState>> {
        self.stack_class.borrow_mut()
    }
}

impl MockParser {
    fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error { span, kind }
    }
    
    fn unclosed_class_error(&self) -> Error {
        for state in self.stack_class.borrow().iter().rev() {
            match *state {
                ClassState::Open { ref set } => {
                    return self.error(set.clone(), ErrorKind::ClassUnclosed);
                }
                _ => {}
            }
        }
        panic!("no open character class found")
    }
}

#[test]
fn test_unclosed_class_error_panics_when_no_class_open() {
    let parser = MockParser::new();

    let result = std::panic::catch_unwind(|| {
        parser.unclosed_class_error();
    });

    assert!(result.is_err());
}

#[test]
fn test_unclosed_class_error_returns_correct_error() {
    let parser = MockParser::new();
    let span = Span { start: 0, end: 5 };
    parser.stack_class.borrow_mut().push(ClassState::Open { set: span });

    let result = parser.unclosed_class_error();

    assert_eq!(result.kind, ErrorKind::ClassUnclosed);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 5);
}

