// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.hole, Hole::None);
    assert_eq!(patch.entry, 0);
}

#[test]
fn test_compile_anchor_start_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(syntax::hir::Anchor::StartLine);

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(patch.entry > 0);
}

#[test]
fn test_compile_anchor_end_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(syntax::hir::Anchor::EndLine);

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(patch.entry > 0);
}

#[test]
fn test_compile_anchor_start_text() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(syntax::hir::Anchor::StartText);

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(patch.entry > 0);
}

#[test]
fn test_compile_anchor_end_text() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(syntax::hir::Anchor::EndText);

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(patch.entry > 0);
}

#[test]
fn test_compile_anchor_negative_word_boundary() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(syntax::hir::WordBoundary::AsciiNegate);

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(patch.entry > 0);
}

#[test]
fn test_compile_anchor_unary_word_boundary() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(syntax::hir::WordBoundary::Ascii);

    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(patch.entry > 0);
}

