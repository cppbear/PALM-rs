// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_word_boundary_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::Unicode);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_word_boundary_ascii() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::Ascii);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_word_boundary_unicode_negate() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::UnicodeNegate);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_word_boundary_ascii_negate() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::AsciiNegate);
    let _ = compiler.c(&expr);
}

