// Answer 0

#[test]
#[should_panic]
fn test_c_alternate_empty() {
    let mut compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![];
    let _ = compiler.c_alternate(&exprs);
}

#[test]
#[should_panic]
fn test_c_alternate_single() {
    let mut compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![Hir::new(hir::Literal::Unicode('a'))];
    let _ = compiler.c_alternate(&exprs);
}

