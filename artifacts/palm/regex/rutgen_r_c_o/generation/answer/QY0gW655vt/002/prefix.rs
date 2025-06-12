// Answer 0

#[test]
fn test_c_alternate_with_empty_sub_expression() {
    use syntax::hir::{Hir, HirKind};

    let mut compiler = Compiler::new();

    let empty_expr = Hir::new(HirKind::Empty);
    let concat_expr = Hir::new(HirKind::Concat(vec![
        Hir::new(HirKind::Literal(hir::Literal::Unicode('a'))),
        Hir::new(HirKind::Literal(hir::Literal::Unicode('b'))),
    ]));

    let exprs = vec![concat_expr, empty_expr];

    let result = compiler.c_alternate(&exprs);
}

#[test]
fn test_c_alternate_with_empty_middle_expression() {
    use syntax::hir::{Hir, HirKind};

    let mut compiler = Compiler::new();

    let empty_expr = Hir::new(HirKind::Empty);
    let concat_expr1 = Hir::new(HirKind::Concat(vec![
        Hir::new(HirKind::Literal(hir::Literal::Unicode('c'))),
    ]));
    let concat_expr2 = Hir::new(HirKind::Concat(vec![
        Hir::new(HirKind::Literal(hir::Literal::Unicode('d'))),
    ]));

    let exprs = vec![concat_expr1, empty_expr, concat_expr2];

    let result = compiler.c_alternate(&exprs);
}

