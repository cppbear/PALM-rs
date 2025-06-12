// Answer 0

#[should_panic]
fn test_compile_empty_exprs() {
    let compiler = Compiler::new();
    let exprs: Vec<Hir> = vec![];
    compiler.compile(&exprs);
}

