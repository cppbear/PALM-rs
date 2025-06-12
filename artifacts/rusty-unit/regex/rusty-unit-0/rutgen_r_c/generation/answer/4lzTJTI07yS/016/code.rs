// Answer 0

#[test]
fn test_compiler_c_with_anchor_end_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(hir::Anchor::EndLine);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_start_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(hir::Anchor::StartLine);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_start_text() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(hir::Anchor::StartText);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_end_text() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(hir::Anchor::EndText);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_end_line_is_reverse_false() {
    let mut compiler = Compiler::new();

    compiler.compiled.is_reverse = false;
    let expr = Hir::new_anchor(hir::Anchor::EndLine);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_start_line_is_reverse_false() {
    let mut compiler = Compiler::new();

    compiler.compiled.is_reverse = false;
    let expr = Hir::new_anchor(hir::Anchor::StartLine);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_start_text_is_reverse_false() {
    let mut compiler = Compiler::new();

    compiler.compiled.is_reverse = false;
    let expr = Hir::new_anchor(hir::Anchor::StartText);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compiler_c_with_anchor_end_text_is_reverse_false() {
    let mut compiler = Compiler::new();

    compiler.compiled.is_reverse = false;
    let expr = Hir::new_anchor(hir::Anchor::EndText);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

