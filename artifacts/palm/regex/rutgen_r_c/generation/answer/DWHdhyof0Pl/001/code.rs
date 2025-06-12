// Answer 0

#[test]
fn test_c_repeat_one_or_more_success() {
    use syntax::hir::{Hir, HirKind, GroupKind};
    use syntax::hir::HirBuilder;

    let mut compiler = Compiler::new();
    let expr = Hir::new(GroupKind::NonCapturing, HirKind::Literal("a".chars().collect()));

    let result = compiler.c_repeat_one_or_more(&expr, true);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_c_repeat_one_or_more_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new(GroupKind::NonCapturing, HirKind::Empty);

    let _ = compiler.c_repeat_one_or_more(&expr, true);
}

#[test]
fn test_c_repeat_one_or_more_non_capturing_group() {
    use syntax::hir::{Hir, HirKind, GroupKind};

    let mut compiler = Compiler::new();
    let expr = Hir::new(GroupKind::NonCapturing, HirKind::Literal("b".chars().collect()));

    let result = compiler.c_repeat_one_or_more(&expr, false);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_c_repeat_one_or_more_invalid_expression() {
    use syntax::hir::HirKind;

    let mut compiler = Compiler::new();
    let expr = Hir::new(GroupKind::NonCapturing, HirKind::Invalid);

    let _ = compiler.c_repeat_one_or_more(&expr, true);
}

