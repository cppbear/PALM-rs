// Answer 0

#[test]
fn test_finish_with_non_empty_stack() {
    let trans = Translator {
        stack: RefCell::new(vec![HirFrame::Expr(Hir::empty())]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, "abc");
    let result = translator_i.finish();
}

#[test]
fn test_finish_with_one_expr_on_stack() {
    let mut stack = Vec::new();
    stack.push(HirFrame::Expr(Hir::empty()));
    let trans = Translator {
        stack: RefCell::new(stack),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, "abc");
    let result = translator_i.finish();
}

#[test]
fn test_finish_with_non_empty_stack_multiple_times() {
    let mut stack = Vec::new();
    stack.push(HirFrame::Expr(Hir::empty()));
    for _ in 0..10 {
        stack.push(HirFrame::Expr(Hir::empty()));
    }
    let trans = Translator {
        stack: RefCell::new(stack),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, "abc");
    let result = translator_i.finish();
}

#[test]
fn test_finish_with_various_expressions_on_stack() {
    let trans = Translator {
        stack: RefCell::new(vec![
            HirFrame::Expr(Hir::empty()),
            HirFrame::Expr(Hir::literal(ast::Literal::new('a'))),
            HirFrame::Expr(Hir::literal(ast::Literal::new('b'))),
        ]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, "abc");
    let result = translator_i.finish();
}

