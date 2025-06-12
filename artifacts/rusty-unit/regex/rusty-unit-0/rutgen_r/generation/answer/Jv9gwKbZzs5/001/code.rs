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
enum ClassState {
    Op { kind: String, lhs: ast::ClassSet },
    Open,
}

mod ast {
    #[derive(Debug)]
    pub struct ClassSet;

    #[derive(Debug)]
    pub struct ClassSetBinaryOp {
        pub span: Span,
        pub kind: String,
        pub lhs: Box<ClassSet>,
        pub rhs: Box<ClassSet>,
    }

    #[derive(Debug)]
    pub struct Span {
        pub start: usize,
        pub end: usize,
    }

    impl ClassSet {
        pub fn span(&self) -> Span {
            Span { start: 0, end: 1 }
        }
    }

    impl super::MockParser {
        pub fn pop_class_op(&self, rhs: ClassSet) -> ClassSet {
            let mut stack = self.stack_class.borrow_mut();
            let (kind, lhs) = match stack.pop() {
                Some(ClassState::Op { kind, lhs }) => (kind, lhs),
                Some(state @ ClassState::Open) => {
                    stack.push(state);
                    return rhs;
                }
                None => unreachable!(),
            };
            let span = Span { start: lhs.span().start, end: rhs.span().end };
            ClassSet::BinaryOp(ClassSetBinaryOp {
                span: span,
                kind: kind,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }
    }
}

#[test]
fn test_pop_class_op_with_operation() {
    let parser = MockParser::new();
    let lhs_class_set = ast::ClassSet;
    parser.stack_class.borrow_mut().push(ClassState::Op { kind: String::from("union"), lhs: lhs_class_set });
    let rhs_class_set = ast::ClassSet;

    let result = parser.pop_class_op(rhs_class_set);

    assert!(matches!(result, ast::ClassSet::BinaryOp { .. }));
}

#[test]
fn test_pop_class_op_with_open() {
    let parser = MockParser::new();
    let rhs_class_set = ast::ClassSet;
    parser.stack_class.borrow_mut().push(ClassState::Open);
    
    let result = parser.pop_class_op(rhs_class_set);

    assert_eq!(result, rhs_class_set);
}

#[test]
#[should_panic]
fn test_pop_class_op_empty_stack() {
    let parser = MockParser::new();
    let rhs_class_set = ast::ClassSet;

    parser.pop_class_op(rhs_class_set);
}

