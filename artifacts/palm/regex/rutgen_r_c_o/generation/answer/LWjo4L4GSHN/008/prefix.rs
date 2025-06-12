// Answer 0

#[test]
fn test_hir_repetition_one_or_more_greedy() {
    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    
    let pattern = "a+";
    let trans_i = TranslatorI::new(&translator, pattern);
    
    let repetition_op = ast::RepetitionOp {
        kind: ast::RepetitionKind::OneOrMore,
        greedy: true,
    };
    let repetition = ast::Repetition {
        span: Span::default(),
        op: repetition_op,
        greedy: true,
        ast: Box::new(ast::Literal::new("a")),
    };
    
    let expr = Hir::literal(hir::Literal::new('a'));
    
    trans_i.hir_repetition(&repetition, expr);
}

#[test]
fn test_hir_repetition_one_or_more_non_greedy() {
    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    
    let pattern = "a+";
    let trans_i = TranslatorI::new(&translator, pattern);
    
    let repetition_op = ast::RepetitionOp {
        kind: ast::RepetitionKind::OneOrMore,
        greedy: false,
    };
    let repetition = ast::Repetition {
        span: Span::default(),
        op: repetition_op,
        greedy: false,
        ast: Box::new(ast::Literal::new("a")),
    };
    
    let expr = Hir::literal(hir::Literal::new('a'));
    
    trans_i.hir_repetition(&repetition, expr);
}

#[test]
fn test_hir_repetition_one_or_more_with_empty_expr() {
    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    
    let pattern = ".*+";
    let trans_i = TranslatorI::new(&translator, pattern);
    
    let repetition_op = ast::RepetitionOp {
        kind: ast::RepetitionKind::OneOrMore,
        greedy: true,
    };
    let repetition = ast::Repetition {
        span: Span::default(),
        op: repetition_op,
        greedy: true,
        ast: Box::new(ast::Literal::new("")),
    };
    
    let expr = Hir::empty();
    
    trans_i.hir_repetition(&repetition, expr);
}

