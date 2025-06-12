// Answer 0

#[test]
fn test_compile_empty() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let expr = Hir::new_class_bytes(hir::ClassBytes::new(vec![]));
    compiler.c(&expr);
}

#[test]
fn test_compile_unicode_literal() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    compiler.c(&expr);
}

#[test]
fn test_compile_byte_literal() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let expr = Hir::new_literal(hir::Literal::Byte(0b10101010));
    compiler.c(&expr);
}

#[test]
fn test_compile_unicode_class() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let ranges = vec![hir::ClassUnicodeRange::new('a', 'z')];
    let expr = Hir::new_class(hir::Class::Unicode(ranges));
    compiler.c(&expr);
}

#[test]
fn test_compile_byte_class() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let ranges = vec![hir::ClassBytesRange::new(0, 255)];
    let expr = Hir::new_class(hir::Class::Bytes(ranges));
    compiler.c(&expr);
}

#[test]
fn test_compile_multiple_byte_literals() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let bytes = vec![0b00000001, 0b00000010];
    let expr = Hir::new_class(hir::Class::Bytes(bytes.iter().map(|&b| hir::ClassBytesRange::new(b, b)).collect()));
    compiler.c(&expr);
}

#[test]
fn test_compile_multiple_unicode_literals() {
    let mut compiler = Compiler::new().size_limit(2097152).bytes(true);
    let expr = Hir::new_class(hir::Class::Unicode(vec![hir::ClassUnicodeRange::new('a', 'c'), hir::ClassUnicodeRange::new('x', 'z')]));
    compiler.c(&expr);
}

