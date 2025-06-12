// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_word_boundary(hir::WordBoundary::Ascii);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_word_boundary() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_word_boundary(hir::WordBoundary::Unicode);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_ascii_negate_word_boundary() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_word_boundary(hir::WordBoundary::AsciiNegate);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_negate_word_boundary() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_word_boundary(hir::WordBoundary::UnicodeNegate);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_start_line_anchor() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_anchor(hir::Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_end_line_anchor() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_anchor(hir::Anchor::EndLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_start_text_anchor() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_anchor(hir::Anchor::StartText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_end_text_anchor() {
    let mut compiler = Compiler::new().size_limit(10 * (1 << 20));
    let expr = Hir::new_anchor(hir::Anchor::EndText);
    let _ = compiler.c(&expr);
}

