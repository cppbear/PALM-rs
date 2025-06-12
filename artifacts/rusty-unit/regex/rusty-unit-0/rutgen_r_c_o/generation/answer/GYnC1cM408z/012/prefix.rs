// Answer 0

#[test]
#[should_panic]
fn test_compile_many_with_empty_exprs() {
    let compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![];
    let _ = compiler.compile_many(&exprs);
}

#[test]
#[should_panic]
fn test_compile_many_with_single_expr() {
    let compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![Hir::any(true)];
    let _ = compiler.compile_many(&exprs);
}

