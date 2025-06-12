// Answer 0

#[test]
fn test_compile_empty_class() {
    let mut compiler = Compiler::new();
    compiler.size_limit(10485760);
    let expr = Hir::new_class(hir::Class::Bytes(hir::ClassBytes::new(vec![])));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_literal() {
    let mut compiler = Compiler::new();
    compiler.size_limit(10485760);
    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_byte_literal() {
    let mut compiler = Compiler::new();
    compiler.size_limit(10485760);
    let expr = Hir::new_literal(hir::Literal::Byte(97)); // 'a'
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_class_with_all_ascii() {
    let mut compiler = Compiler::new();
    compiler.size_limit(10485760);
    let class = hir::ClassBytes::new(vec![hir::ClassBytesRange::new(97, 122)]); // 'a' to 'z'
    let expr = Hir::new_class(hir::Class::Bytes(class));
    compiler.compiled.is_bytes = false; // Ensure uses_bytes is false
    let _ = compiler.c(&expr);
}

