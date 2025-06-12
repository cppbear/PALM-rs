// Answer 0

fn test_compile_empty() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

fn test_compile_literal_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

fn test_compile_literal_byte() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_literal(hir::Literal::Byte(b'a'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

fn test_compile_word_boundary_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::Unicode);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

fn test_compile_word_boundary_unicode_negate() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::UnicodeNegate);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

fn test_compile_word_boundary_ascii() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::Ascii);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

fn test_compile_word_boundary_ascii_negate() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary(hir::WordBoundary::AsciiNegate);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

