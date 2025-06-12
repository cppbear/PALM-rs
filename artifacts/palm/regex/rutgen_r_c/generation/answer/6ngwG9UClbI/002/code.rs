// Answer 0

#[test]
fn test_ast_span_alternation() {
    struct DummyHir;
    struct DummyClass {
        span: Span,
    }
    // Implementing a span function for DummyClass
    impl DummyClass {
        pub fn span(&self) -> &Span {
            &self.span
        }
    }

    struct DummyAst {
        span: Span,
    }
    // Implementing a span function for DummyAst
    impl DummyAst {
        pub fn span(&self) -> &Span {
            &self.span
        }
    }

    let span1 = Span { start: 0, end: 5 };
    let span2 = Span { start: 6, end: 10 };
    let alternation = Ast::Alternation(Alternation {
        span: span1.clone(),
        asts: vec![
            Box::new(DummyAst { span: span2 }),
            Box::new(DummyAst { span: span1 }),
        ],
    });

    assert_eq!(alternation.span(), &span1);
}

#[test]
fn test_ast_span_group() {
    let span = Span { start: 0, end: 5 };
    let group = Ast::Group(Group {
        span: span.clone(),
        kind: GroupKind::Capturing(0),
        hir: Box::new(Ast::Empty(span.clone())),
    });

    assert_eq!(group.span(), &span);
}

#[test]
fn test_ast_span_repetition() {
    let span = Span { start: 0, end: 5 };
    let repetition = Ast::Repetition(Repetition {
        span: span.clone(),
        op: RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(Ast::Empty(span.clone())),
    });

    assert_eq!(repetition.span(), &span);
}

#[test]
fn test_ast_span_concat() {
    let span = Span { start: 0, end: 5 };
    let concat = Ast::Concat(Concat {
        span: span.clone(),
        asts: vec![
            Box::new(Ast::Empty(span.clone())),
            Box::new(Ast::Empty(span.clone())),
        ],
    });

    assert_eq!(concat.span(), &span);
}

#[test]
fn test_ast_span_literal() {
    let span = Span { start: 0, end: 1 };
    let literal = Ast::Literal(Literal {
        span: span.clone(),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    });

    assert_eq!(literal.span(), &span);
}

