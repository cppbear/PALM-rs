// Answer 0

#[test]
fn test_compile_anchor_end_line() {
    let mut compiler = Compiler::new().size_limit(1048576).dfa(false).reverse(false);
    let expr = Hir::new(Anchor::EndLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_literal_unicode() {
    let mut compiler = Compiler::new().size_limit(1048576).dfa(false).reverse(false);
    let expr = Hir::new(Literal::Unicode('a'));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_literal_byte() {
    let mut compiler = Compiler::new().size_limit(1048576).dfa(false).reverse(false);
    let expr = Hir::new(Literal::Byte(b'a'));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_anchor_start_line() {
    let mut compiler = Compiler::new().size_limit(1048576).dfa(false).reverse(false);
    let expr = Hir::new(Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_anchor_start_text() {
    let mut compiler = Compiler::new().size_limit(1048576).dfa(false).reverse(false);
    let expr = Hir::new(Anchor::StartText);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_anchor_end_text() {
    let mut compiler = Compiler::new().size_limit(1048576).dfa(false).reverse(false);
    let expr = Hir::new(Anchor::EndText);
    let _ = compiler.c(&expr);
}

