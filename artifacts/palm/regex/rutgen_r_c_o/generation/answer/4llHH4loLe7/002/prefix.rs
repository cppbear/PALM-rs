// Answer 0

#[test]
fn test_visit_post_with_concat() {
    let mut translator = Translator::default();
    let pattern = "ab|cd";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: 0, end: 2 };
    let ast_concat = Ast::Concat(vec![
        Ast::Literal(Literal { span, kind: LiteralKind::Char, c: 'a' }),
        Ast::Literal(Literal { span, kind: LiteralKind::Char, c: 'b' })
    ]);
    
    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));
    
    let result = visitor.visit_post(&ast_concat);
    assert!(result.is_ok());
}

#[test]
fn test_visit_post_with_multiple_exprs() {
    let mut translator = Translator::default();
    let pattern = "x{3,5}|y?";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: 0, end: 5 };
    let ast_concat = Ast::Concat(vec![
        Ast::Repetition(Repetition { span, op: RepetitionOp::Range(RepetitionRange::Bounded(3, 5)), greedy: true, ast: Box::new(Ast::Literal(Literal { span, kind: LiteralKind::Char, c: 'x' })) }),
        Ast::Repetition(Repetition { span, op: RepetitionOp::ZeroOrOne, greedy: true, ast: Box::new(Ast::Literal(Literal { span, kind: LiteralKind::Char, c: 'y' })) })
    ]);
    
    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));
    
    let result = visitor.visit_post(&ast_concat);
    assert!(result.is_ok());
}

#[test]
fn test_visit_post_with_empty_exprs() {
    let mut translator = Translator::default();
    let pattern = "(?:)";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span = Span { start: 0, end: 1 };
    let ast_concat = Ast::Concat(vec![
        Ast::Empty(span),
        Ast::Empty(span)
    ]);
    
    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));
    
    let result = visitor.visit_post(&ast_concat);
    assert!(result.is_ok());
}

#[test]
fn test_visit_post_with_mixed_exprs() {
    let mut translator = Translator::default();
    let pattern = "a|b|c";
    let mut visitor = TranslatorI::new(&translator, pattern);
    
    let span_a = Span { start: 0, end: 1 };
    let span_b = Span { start: 1, end: 2 };
    let span_c = Span { start: 2, end: 3 };
    
    let ast_concat = Ast::Concat(vec![
        Ast::Literal(Literal { span: span_a, kind: LiteralKind::Char, c: 'a' }),
        Ast::Literal(Literal { span: span_b, kind: LiteralKind::Char, c: 'b' }),
        Ast::Literal(Literal { span: span_c, kind: LiteralKind::Char, c: 'c' })
    ]);

    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));
    visitor.push(HirFrame::Expr(Hir::empty()));
    
    let result = visitor.visit_post(&ast_concat);
    assert!(result.is_ok());
}

