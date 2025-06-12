// Answer 0

#[test]
fn test_induct_repetition() {
    use ast::{Ast, Repetition, Span, Class, ClassBracketed};
    
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &ClassBracketed) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(0, 1);
    let ast_repetition = Ast::Repetition(Repetition {
        span,
        op: RepetitionOp::Star,
        greedy: true,
        ast: Box::new(Ast::Empty(span)),
    });

    let mut visitor = DummyVisitor;
    let mut visitor_instance = HeapVisitor::new();
    
    let result = visitor_instance.induct(&ast_repetition, &mut visitor);
    
    assert_eq!(result, Ok(Some(Frame::Repetition(&ast_repetition.repetition))));
}

#[test]
fn test_induct_group() {
    use ast::{Ast, Group, Span};
    
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &ClassBracketed) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(0, 1);
    let ast_group = Ast::Group(Group {
        span,
        kind: GroupKind::Capture,
        ast: Box::new(Ast::Empty(span)),
    });

    let mut visitor = DummyVisitor;
    let mut visitor_instance = HeapVisitor::new();

    let result = visitor_instance.induct(&ast_group, &mut visitor);
    
    assert_eq!(result, Ok(Some(Frame::Group(&ast_group.group))));
}

#[test]
fn test_induct_concat_empty() {
    use ast::{Ast, Concat, Span};

    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &ClassBracketed) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(0, 1);
    let ast_concat_empty = Ast::Concat(Concat {
        span,
        asts: Vec::new(),
    });

    let mut visitor = DummyVisitor;
    let mut visitor_instance = HeapVisitor::new();

    let result = visitor_instance.induct(&ast_concat_empty, &mut visitor);
    
    assert_eq!(result, Ok(None));
}

#[test]
fn test_induct_concat() {
    use ast::{Ast, Concat, Span};

    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &ClassBracketed) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(0, 1);
    let ast_concat = Ast::Concat(Concat {
        span,
        asts: vec![Ast::Empty(span), Ast::Empty(span)],
    });

    let mut visitor = DummyVisitor;
    let mut visitor_instance = HeapVisitor::new();

    let result = visitor_instance.induct(&ast_concat, &mut visitor);
    
    assert_eq!(result, Ok(Some(Frame::Concat {
        head: &ast_concat.concat.asts[0],
        tail: &ast_concat.concat.asts[1..],
    })));
} 

#[test]
fn test_induct_alternation_empty() {
    use ast::{Ast, Alternation, Span};

    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &ClassBracketed) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(0, 1);
    let ast_alternation_empty = Ast::Alternation(Alternation {
        span,
        asts: Vec::new(),
    });

    let mut visitor = DummyVisitor;
    let mut visitor_instance = HeapVisitor::new();

    let result = visitor_instance.induct(&ast_alternation_empty, &mut visitor);
    
    assert_eq!(result, Ok(None));
}

#[test]
fn test_induct_alternation() {
    use ast::{Ast, Alternation, Span};

    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &ClassBracketed) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(0, 1);
    let ast_alternation = Ast::Alternation(Alternation {
        span,
        asts: vec![Ast::Empty(span), Ast::Empty(span)],
    });

    let mut visitor = DummyVisitor;
    let mut visitor_instance = HeapVisitor::new();

    let result = visitor_instance.induct(&ast_alternation, &mut visitor);
    
    assert_eq!(result, Ok(Some(Frame::Alternation {
        head: &ast_alternation.alternation.asts[0],
        tail: &ast_alternation.alternation.asts[1..],
    })));
}

