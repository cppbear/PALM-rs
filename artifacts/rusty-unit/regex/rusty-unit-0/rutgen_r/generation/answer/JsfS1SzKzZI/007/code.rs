// Answer 0

#[test]
fn test_visit_class_with_inductive_case() {
    struct TestVisitor {
        visited: Vec<String>,
    }

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class_pre(&mut self, _ast: &ClassInduct, _visitor: &mut Self) -> Result<(), Self::Err> {
            self.visited.push("pre".to_string());
            Ok(())
        }

        fn visit_class_post(&mut self, _ast: &ClassInduct, _visitor: &mut Self) -> Result<(), Self::Err> {
            self.visited.push("post".to_string());
            Ok(())
        }

        fn visit_class_set_binary_op_in(&mut self, _op: &str) -> Result<(), Self::Err> {
            self.visited.push("binary_op".to_string());
            Ok(())
        }
    }

    struct TestInductor {
        stack_class: Vec<(ClassInduct, ClassFrame)>,
    }

    impl TestInductor {
        fn visit_class_pre(&mut self, ast: &ClassInduct, visitor: &mut TestVisitor) -> Result<(), ()> {
            visitor.visit_class_pre(ast, visitor)
        }

        fn visit_class_post(&mut self, ast: &ClassInduct, visitor: &mut TestVisitor) -> Result<(), ()> {
            visitor.visit_class_post(ast, visitor)
        }

        fn induct_class(&mut self, _ast: &ClassInduct) -> Option<InductiveCase> {
            Some(InductiveCase::new())
        }

        fn pop_class(&mut self, frame: ClassFrame) -> Option<ClassFrame> {
            self.stack_class.pop().map(|(_, f)| f)
        }
    }

    #[derive(Debug)]
    struct ClassInduct;

    impl ClassInduct {
        fn from_bracketed(_ast: &ast::ClassBracketed) -> Self {
            ClassInduct
        }
    }

    #[derive(Debug)]
    struct InductiveCase;

    impl InductiveCase {
        fn new() -> Self {
            InductiveCase
        }

        fn child(&self) -> ClassInduct {
            ClassInduct
        }
    }

    #[derive(Debug)]
    enum ClassFrame {
        BinaryRHS { op: String },
    }

    let mut visitor = TestVisitor { visited: vec![] };
    let mut inducer = TestInductor { stack_class: vec![] };
    let ast = ast::ClassBracketed;

    let result = inducer.visit_class(&ast, &mut visitor);
    
    assert!(result.is_ok());
    assert_eq!(visitor.visited, vec!["pre", "post", "binary_op"]);
}

