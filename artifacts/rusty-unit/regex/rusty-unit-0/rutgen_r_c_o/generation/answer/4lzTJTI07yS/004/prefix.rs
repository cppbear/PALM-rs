// Answer 0

#[test]
fn test_compile_concat_normal() {
    let mut compiler = Compiler::new();
    compiler.size_limit(20971520);
    let hir1 = Hir::new_literal(hir::Literal::Unicode('a'));
    let hir2 = Hir::new_literal(hir::Literal::Unicode('b'));
    let hir_concat = Hir::new_concat(vec![hir1.clone(), hir2.clone()]);
    let _ = compiler.c(&hir_concat);
}

#[test]
fn test_compile_concat_empty() {
    let mut compiler = Compiler::new();
    compiler.size_limit(20971520);
    let hir_concat = Hir::new_concat(vec![]);
    let _ = compiler.c(&hir_concat);
}

#[test]
fn test_compile_concat_multiple() {
    let mut compiler = Compiler::new();
    compiler.size_limit(20971520);
    let mut exprs = vec![];
    for ch in 'c'..='l' {
        exprs.push(Hir::new_literal(hir::Literal::Unicode(ch)));
    }
    let hir_concat = Hir::new_concat(exprs);
    let _ = compiler.c(&hir_concat);
}

#[test]
fn test_compile_concat_reverse_disabled() {
    let mut compiler = Compiler::new();
    compiler.size_limit(20971520);
    compiler.compiled.is_reverse = false;
    let hir1 = Hir::new_literal(hir::Literal::Unicode('x'));
    let hir2 = Hir::new_literal(hir::Literal::Unicode('y'));
    let hir_concat = Hir::new_concat(vec![hir1.clone(), hir2.clone()]);
    let _ = compiler.c(&hir_concat);
} 

#[test]
fn test_compile_concat_large() {
    let mut compiler = Compiler::new();
    compiler.size_limit(20971520);
    let mut exprs = vec![];
    for ch in 'a'..='j' {
        exprs.push(Hir::new_literal(hir::Literal::Unicode(ch)));
    }
    let hir_concat = Hir::new_concat(exprs);
    let _ = compiler.c(&hir_concat);
} 

#[test]
#[should_panic]
fn test_compile_concat_exceed_size() {
    let mut compiler = Compiler::new();
    compiler.size_limit(0);
    let hir1 = Hir::new_literal(hir::Literal::Unicode('p'));
    let hir2 = Hir::new_literal(hir::Literal::Unicode('q'));
    let hir_concat = Hir::new_concat(vec![hir1.clone(), hir2.clone()]);
    let _ = compiler.c(&hir_concat);
}

#[test]
fn test_compile_concat_edge_case() {
    let mut compiler = Compiler::new();
    compiler.size_limit(20971520);
    let hir1 = Hir::new_empty();
    let hir2 = Hir::new_literal(hir::Literal::Unicode('z'));
    let hir_concat = Hir::new_concat(vec![hir1.clone(), hir2.clone()]);
    let _ = compiler.c(&hir_concat);
}

