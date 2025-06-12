// Answer 0

#[test]
fn test_unclosed_class_error_with_no_open_class() {
    struct TestParser {
        stack_class: RefCell<Vec<ClassState>>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                stack_class: RefCell::new(vec![]),
            }
        }
        
        fn stack_class(&self) -> &RefCell<Vec<ClassState>> {
            &self.stack_class
        }
    }
    
    let parser = TestParser::new();
    
    let result = std::panic::catch_unwind(|| {
        let _ = parser.stack_class.borrow().iter().rev().find(|_| true);
    });

    assert!(result.is_err());
}

#[test]
fn test_unclosed_class_error_with_open_class() {
    struct TestParser {
        stack_class: RefCell<Vec<ClassState>>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                stack_class: RefCell::new(vec![
                    ClassState::Open {
                        union: ast::ClassSetUnion::default(),
                        set: ast::ClassBracketed {
                            span: Span { start: 0, end: 1 },
                            negated: false,
                            kind: ast::ClassSet::default(),
                        },
                    },
                    ClassState::Open {
                        union: ast::ClassSetUnion::default(),
                        set: ast::ClassBracketed {
                            span: Span { start: 2, end: 3 },
                            negated: false,
                            kind: ast::ClassSet::default(),
                        },
                    },
                ]),
            }
        }

        fn stack_class(&self) -> &RefCell<Vec<ClassState>> {
            &self.stack_class
        }
    }
    
    let parser = TestParser::new();
    
    let result = std::panic::catch_unwind(|| {
        for state in parser.stack_class.borrow().iter().rev() {
            match state {
                ClassState::Open { set, .. } => {
                    return parser.error(set.span, ast::ErrorKind::ClassUnclosed);
                },
                _ => {}
            }
        }
    });

    assert!(result.is_err());
}

