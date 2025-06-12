// Answer 0

#[test]
fn test_c_repeat_range_min_1_max_5_greedy_false() {
    use syntax::hir::{Hir, HirKind, Class, Literal};
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Literal(Literal::Unicode('a')));
    compiler.c_concat(std::iter::once(&expr)).unwrap();
    let result = compiler.c_repeat_range(&expr, false, 1, 5);
}

#[test]
fn test_c_repeat_range_min_2_max_4_greedy_false() {
    use syntax::hir::{Hir, HirKind, Class, Literal};
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Literal(Literal::Unicode('b')));
    compiler.c_concat(std::iter::once(&expr)).unwrap();
    let result = compiler.c_repeat_range(&expr, false, 2, 4);
}

#[test]
fn test_c_repeat_range_min_1_max_3_greedy_false() {
    use syntax::hir::{Hir, HirKind, Class, Literal};
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Class(Class::Unicode(vec![])));
    compiler.c_concat(std::iter::once(&expr)).unwrap();
    let result = compiler.c_repeat_range(&expr, false, 1, 3);
}

#[test]
fn test_c_repeat_range_min_3_max_6_greedy_false() {
    use syntax::hir::{Hir, HirKind, Class, Literal};
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Class(Class::Unicode(vec![])));
    compiler.c_concat(std::iter::once(&expr)).unwrap();
    let result = compiler.c_repeat_range(&expr, false, 3, 6);
}

#[test]
fn test_c_repeat_range_min_1_max_2_greedy_false() {
    use syntax::hir::{Hir, HirKind, Class, Literal};
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Class(Class::Unicode(vec![])));
    compiler.c_concat(std::iter::once(&expr)).unwrap();
    let result = compiler.c_repeat_range(&expr, false, 1, 2);
}

#[test]
fn test_c_repeat_range_min_1_max_6_greedy_false() {
    use syntax::hir::{Hir, HirKind, Class, Literal};
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Literal(Literal::Unicode('c')));
    compiler.c_concat(std::iter::once(&expr)).unwrap();
    let result = compiler.c_repeat_range(&expr, false, 1, 6);
}

