// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_class() {
    let mut compiler = Compiler::new();
    let ranges = vec![hir::ClassUnicodeRange::new('a', 'z'), hir::ClassUnicodeRange::new('A', 'Z')];
    let expr = Hir::new_class(hir::Class::Unicode(ranges));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_bytes_class() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = vec![hir::ClassBytesRange::new(0x00, 0x7F)];
    let expr = Hir::new_class(hir::Class::Bytes(ranges));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_empty_look_start_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_anchor(hir::Anchor::StartLine);
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_repetition_zero_or_more() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_repetition(hir::Repetition::new_zero_or_more(Hir::new_literal('a')));
    let _ = compiler.c(&expr);
}

