// Answer 0

#[test]
fn test_visit_class() {
    struct MockVisitor {
        visit_class_set_binary_op_in_called: bool,
        err: Option<String>,
    }

    impl Visitor for MockVisitor {
        type Err = String;

        fn visit_class_set_binary_op_in(&mut self, _op: &str) -> Result<(), Self::Err> {
            self.visit_class_set_binary_op_in_called = true;
            Ok(())
        }

        fn visit_class_pre(&mut self, _ast: &ClassInduct, _visitor: &mut Self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_class_post(&mut self, _ast: &ClassInduct, _visitor: &mut Self) -> Result<(), Self::Err> {
            self.err.clone().map_or(Ok(()), Err)
        }
    }

    struct MockClassFrame {}

    impl MockClassFrame {
        fn child(&self) -> ClassInduct {
            ClassInduct::new()
        }
    }

    struct MockSelf {
        stack_class: Vec<(ClassInduct, MockClassFrame)>,
    }

    impl MockSelf {
        fn visit_class_pre(&mut self, ast: &ClassInduct, visitor: &mut MockVisitor) -> Result<(), String> {
            visitor.visit_class_pre(ast, visitor)
        }

        fn visit_class_post(&mut self, ast: &ClassInduct, visitor: &mut MockVisitor) -> Result<(), String> {
            visitor.visit_class_post(ast, visitor)
        }

        fn induct_class(&mut self, _ast: &ClassInduct) -> Option<MockClassFrame> {
            Some(MockClassFrame {})
        }

        fn pop_class(&mut self, _frame: MockClassFrame) -> Option<MockClassFrame> {
            Some(MockClassFrame {})
        }

        fn visit_class(&mut self, ast: &ClassInduct, visitor: &mut MockVisitor) -> Result<(), String> {
            let mut ast = ast.clone();
            loop {
                self.visit_class_pre(&ast, visitor)?;
                if let Some(x) = self.induct_class(&ast) {
                    let child = x.child();
                    self.stack_class.push((ast.clone(), x));
                    ast = child;
                    continue;
                }
                self.visit_class_post(&ast, visitor)?;

                loop {
                    let (post_ast, frame) = match self.stack_class.pop() {
                        None => return Ok(()),
                        Some((post_ast, frame)) => (post_ast, frame),
                    };
                    if let Some(x) = self.pop_class(frame) {
                        ast = x.child();
                        self.stack_class.push((post_ast, x));
                        break;
                    }
                    self.visit_class_post(&post_ast, visitor)?;
                }
            }
        }
    }

    let mut visitor = MockVisitor {
        visit_class_set_binary_op_in_called: false,
        err: Some("Error".to_string()),
    };

    let mut mock_self = MockSelf {
        stack_class: Vec::new(),
    };

    let ast = ClassInduct::new();

    let result = mock_self.visit_class(&ast, &mut visitor);
    assert_eq!(result, Err("Error".to_string()));
    assert!(visitor.visit_class_set_binary_op_in_called);
}

