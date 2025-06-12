// Answer 0

#[test]
fn test_visit_with_valid_visitor() {
    struct TestVisitor {
        result: Vec<u32>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<u32>;
        type Err = ();

        fn visit_node(&mut self, node: &Hir) -> Result<(), Self::Err> {
            self.result.push(node.id); // Assuming nodes have an id for testing
            Ok(())
        }
    }

    let hir = Hir::new(); // Replace with actual initialization
    let visitor = TestVisitor { result: Vec::new() };

    let result = visit(&hir, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![/* expected node ids */]);
}

#[test]
fn test_visit_with_erroring_visitor() {
    struct ErringVisitor;

    impl Visitor for ErringVisitor {
        type Output = ();
        type Err = String;

        fn visit_node(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Err("Visitor error".to_string())
        }
    }

    let hir = Hir::new(); // Replace with actual initialization
    let visitor = ErringVisitor;

    let result = visit(&hir, visitor);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Visitor error".to_string());
}

