// Answer 0

#[test]
fn test_pop_class_op_with_operation() {
    struct MockParser {
        stack_class: std::cell::RefCell<Vec<ClassState>>,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_class: std::cell::RefCell::new(vec![]),
            }
        }
    }

    struct MockClass {
        parser: MockParser,
    }

    impl MockClass {
        fn parser(&self) -> &MockParser {
            &self.parser
        }
    }

    struct MockClassSet {
        span: Span,
    }

    impl MockClassSet {
        fn new(start: usize, end: usize) -> Self {
            Self {
                span: Span::new(start, end),
            }
        }
        
        fn span(&self) -> &Span {
            &self.span
        }
    }

    let mut mock_parser = MockParser::new();
    mock_parser.stack_class.borrow_mut().push(ClassState::Op { kind: BinaryOpKind::Union, lhs: MockClassSet::new(0, 1) });
    
    let mock_class = MockClass { parser: mock_parser };
    let rhs = MockClassSet::new(2, 3);
    
    let result = mock_class.pop_class_op(rhs);
    
    match result {
        ast::ClassSet::BinaryOp(op) => {
            assert_eq!(op.kind, BinaryOpKind::Union);
            assert_eq!(op.lhs.span().start, 0);
            assert_eq!(op.rhs.span().end, 3);
        },
        _ => panic!("Expected a BinaryOp, but got a different type."),
    }
}

#[test]
fn test_pop_class_op_without_operation() {
    struct MockParser {
        stack_class: std::cell::RefCell<Vec<ClassState>>,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                stack_class: std::cell::RefCell::new(vec![]),
            }
        }
    }

    struct MockClass {
        parser: MockParser,
    }

    impl MockClass {
        fn parser(&self) -> &MockParser {
            &self.parser
        }
    }

    struct MockClassSet {
        span: Span,
    }

    impl MockClassSet {
        fn new(start: usize, end: usize) -> Self {
            Self {
                span: Span::new(start, end),
            }
        }
        
        fn span(&self) -> &Span {
            &self.span
        }
    }

    let mut mock_parser = MockParser::new();
    mock_parser.stack_class.borrow_mut().push(ClassState::Open { /* fields as required */ });
    
    let mock_class = MockClass { parser: mock_parser };
    let rhs = MockClassSet::new(2, 3);
    
    let result = mock_class.pop_class_op(rhs);
    
    assert_eq!(result.span().start, 2);
    assert_eq!(result.span().end, 3);
}

