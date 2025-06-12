// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_literal_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_literal_unicode('a');
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_literal_byte() {
    let mut compiler = Compiler::new();
    compiler.bytes(true);
    let expr = Hir::new_literal_byte(b'a');
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_class_unicode() {
    let mut compiler = Compiler::new();
    let class_ranges = vec![hir::ClassUnicodeRange::new('a', 'z')];
    let expr = Hir::new_class_unicode(class_ranges);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_class_bytes() {
    let mut compiler = Compiler::new();
    let bytes_ranges = vec![hir::ClassBytesRange::new(0, 255)];
    compiler.bytes(true);
    let expr = Hir::new_class_bytes(bytes_ranges);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_class_bytes_non_ascii() {
    let mut compiler = Compiler::new();
    let bytes_ranges = vec![hir::ClassBytesRange::new(128, 255)]; // Non-ASCII
    compiler.bytes(true);
    let expr = Hir::new_class_bytes(bytes_ranges);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_compile_class_unicode_with_is_all_ascii() {
    let mut compiler = Compiler::new();
    let class_ranges = vec![hir::ClassUnicodeRange::new('A', 'Z')]; // All ASCII
    let expr = Hir::new_class_unicode(class_ranges);
    let result = compiler.c(&expr);
    assert!(result.is_err());
}

#[test]
fn test_compile_word_boundary() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_word_boundary_unicode();
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_group_capture_index() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_group_capture_index(0, Hir::new_literal_unicode('b'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_concatenation() {
    let mut compiler = Compiler::new();
    let expr1 = Hir::new_literal_unicode('c');
    let expr2 = Hir::new_literal_unicode('d');
    let expr = Hir::new_concatenation(vec![expr1, expr2]);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_alternation() {
    let mut compiler = Compiler::new();
    let expr1 = Hir::new_literal_unicode('e');
    let expr2 = Hir::new_literal_unicode('f');
    let expr = Hir::new_alternation(vec![expr1, expr2]);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

