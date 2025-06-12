// Answer 0

#[test]
fn test_visit_post_concat_with_non_empty_expression() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let mut visitor = TranslatorI::new(&trans, "test_pattern");

    let span = Span { start: 0, end: 5 };
    let expr1 = Hir::literal(Literal { span, kind: LiteralKind::Unicode, c: 'a' });
    let expr2 = Hir::literal(Literal { span, kind: LiteralKind::Unicode, c: 'b' });
    let expr3 = Hir::empty();

    visitor.push(HirFrame::Expr(expr2)); 
    visitor.push(HirFrame::Expr(expr1)); 
    visitor.push(HirFrame::Expr(expr3)); 

    let ast = Ast::Concat(vec![]); 
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_concat_with_empty_expression() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let mut visitor = TranslatorI::new(&trans, "test_pattern");

    let span = Span { start: 0, end: 5 };
    let expr1 = Hir::literal(Literal { span, kind: LiteralKind::Unicode, c: 'c' });
    let expr2 = Hir::literal(Literal { span, kind: LiteralKind::Unicode, c: 'd' });
    let expr3 = Hir::empty();

    visitor.push(HirFrame::Expr(expr3)); 
    visitor.push(HirFrame::Expr(expr2)); 
    visitor.push(HirFrame::Expr(expr1)); 

    let ast = Ast::Concat(vec![]); 
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_concat_popping_empty_expression() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let mut visitor = TranslatorI::new(&trans, "test_pattern");

    let span = Span { start: 0, end: 5 };
    let expr1 = Hir::literal(Literal { span, kind: LiteralKind::Unicode, c: 'e' });
    let expr2 = Hir::literal(Literal { span, kind: LiteralKind::Unicode, c: 'f' });

    visitor.push(HirFrame::Expr(expr2)); 
    visitor.push(HirFrame::Expr(expr1)); 

    let ast = Ast::Concat(vec![]); 
    visitor.visit_post(&ast).unwrap();
}

