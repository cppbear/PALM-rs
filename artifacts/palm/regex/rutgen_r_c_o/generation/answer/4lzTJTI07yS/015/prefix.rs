// Answer 0

#[test]
fn test_c_empty_look_anchor_end_line() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(true);
    let expr = Hir::new(hir::Anchor::EndLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_start_line() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(true);
    let expr = Hir::new(hir::Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_start_text() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(true);
    let expr = Hir::new(hir::Anchor::StartText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_end_text() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20)).reverse(true);
    let expr = Hir::new(hir::Anchor::EndText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_end_line_with_min_size() {
    let mut compiler = Compiler::new().size_limit(1).reverse(true);
    let expr = Hir::new(hir::Anchor::EndLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_start_line_with_min_size() {
    let mut compiler = Compiler::new().size_limit(1).reverse(true);
    let expr = Hir::new(hir::Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_start_text_with_min_size() {
    let mut compiler = Compiler::new().size_limit(1).reverse(true);
    let expr = Hir::new(hir::Anchor::StartText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_c_empty_look_anchor_end_text_with_min_size() {
    let mut compiler = Compiler::new().size_limit(1).reverse(true);
    let expr = Hir::new(hir::Anchor::EndText);
    let _ = compiler.c(&expr);
}

