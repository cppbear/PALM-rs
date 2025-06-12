// Answer 0

#[test]
fn test_compile_empty() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_literal() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_byte_literal() {
    let mut compiler = Compiler::new();
    compiler = compiler.bytes(true);
    let expr = Hir::new_literal(hir::Literal::Byte(10));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_unicode_class() {
    let mut compiler = Compiler::new();
    let unicode_ranges = vec![hir::ClassUnicodeRange::new('a', 'z')];
    let expr = Hir::new_class(hir::Class::Unicode(unicode_ranges));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_bytes_class() {
    let mut compiler = Compiler::new();
    compiler = compiler.bytes(true);
    let bytes_ranges = vec![hir::ClassBytesRange::new(128, 255)];
    let expr = Hir::new_class(hir::Class::Bytes(bytes_ranges));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_mixed_unicode_class() {
    let mut compiler = Compiler::new();
    let unicode_ranges = vec![hir::ClassUnicodeRange::new('a', 'z'), hir::ClassUnicodeRange::new('ñ', 'ñ')];
    let expr = Hir::new_class(hir::Class::Unicode(unicode_ranges));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_non_ascii_unicode_class() {
    let mut compiler = Compiler::new();
    let unicode_ranges = vec![hir::ClassUnicodeRange::new('一', '中')];
    let expr = Hir::new_class(hir::Class::Unicode(unicode_ranges));
    let _ = compiler.c(&expr);
}

#[test]
#[should_panic]
fn test_compile_bytes_class_not_supported() {
    let mut compiler = Compiler::new();
    let bytes_ranges = vec![hir::ClassBytesRange::new(0, 255)];
    let expr = Hir::new_class(hir::Class::Bytes(bytes_ranges));
    let _ = compiler.c(&expr);
}

#[test]
fn test_compile_combined_classes() {
    let mut compiler = Compiler::new();
    let unicode_ranges = vec![hir::ClassUnicodeRange::new('a', 'z')];
    let bytes_ranges = vec![hir::ClassBytesRange::new(100, 200)];
    let expr = Hir::new_concat(vec![
        Hir::new_class(hir::Class::Unicode(unicode_ranges)),
        Hir::new_class(hir::Class::Bytes(bytes_ranges)),
    ]);
    let _ = compiler.c(&expr);
}

