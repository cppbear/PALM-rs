// Answer 0

#[test]
fn test_c_compile_anchor_start_line() {
    let mut compiler = Compiler::new().size_limit(1024).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_end_line() {
    let mut compiler = Compiler::new().size_limit(1024).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::EndLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_start_text() {
    let mut compiler = Compiler::new().size_limit(1024).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::StartText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_end_text() {
    let mut compiler = Compiler::new().size_limit(1024).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::EndText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_end_text_with_large_size_limit() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::EndText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_start_line_with_large_size_limit() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_end_line_with_large_size_limit() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::EndLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_compile_anchor_start_text_with_large_size_limit() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(false);
    let expr = Hir::new_anchor(hir::Anchor::StartText);
    let _ = compiler.c(&expr);
}

