// Answer 0

#[test]
fn test_compile_anchor_start_line_reverse() {
    let mut compiler = Compiler::new().reverse(true);
    let expr = Hir::new(Anchor(hir::Anchor::StartLine));
    let result = compiler.c(&expr);
    
    assert!(result.is_ok());
}

#[test]
fn test_compile_anchor_end_line_reverse() {
    let mut compiler = Compiler::new().reverse(true);
    let expr = Hir::new(Anchor(hir::Anchor::EndLine));
    let result = compiler.c(&expr);
    
    assert!(result.is_ok());
}

#[test]
fn test_compile_anchor_start_text_reverse() {
    let mut compiler = Compiler::new().reverse(true);
    let expr = Hir::new(Anchor(hir::Anchor::StartText));
    let result = compiler.c(&expr);
    
    assert!(result.is_ok());
}

#[test]
fn test_compile_anchor_end_text_reverse() {
    let mut compiler = Compiler::new().reverse(true);
    let expr = Hir::new(Anchor(hir::Anchor::EndText));
    let result = compiler.c(&expr);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_compile_anchor_end_text_exceed_size_limit() {
    let mut compiler = Compiler::new().size_limit(0).reverse(true);
    let expr = Hir::new(Anchor(hir::Anchor::EndText));
    
    let _ = compiler.c(&expr);
}

