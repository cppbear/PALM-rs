// Answer 0

#[test]
fn test_induct_alternation_non_empty() {
    struct DummyVisitor {
        output: usize,
    }

    impl Visitor for DummyVisitor {
        type Output = usize;
        type Err = ();

        fn visit(&mut self, _ast: &Ast) -> Result<Self::Output, Self::Err> {
            self.output += 1;
            Ok(self.output)
        }
    }

    let ast1 = Ast::Alternation(Alternation {
        span: Span { start: 0, end: 1 },
        asts: vec![
            Ast::Literal(Literal { value: 'a' }),
            Ast::Literal(Literal { value: 'b' }),
        ],
    });

    let ast2 = Ast::Alternation(Alternation {
        span: Span { start: 1, end: 2 },
        asts: vec![
            Ast::Literal(Literal { value: 'c' }),
        ],
    });

    let mut visitor = DummyVisitor { output: 0 };
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast1, &mut visitor).unwrap();
    assert!(result.is_some());

    if let Some(Frame::Alternation { head, tail }) = result {
        assert_eq!(head, &ast1.asts[0]);
        assert_eq!(tail, &ast1.asts[1..]);
    } else {
        panic!("Expected Alternation frame");
    }

    let result = heap_visitor.induct(&ast2, &mut visitor).unwrap();
    assert!(result.is_some());

    if let Some(Frame::Alternation { head, tail }) = result {
        assert_eq!(head, &ast2.asts[0]);
        assert!(tail.is_empty());
    } else {
        panic!("Expected Alternation frame");
    }
}

