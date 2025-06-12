// Answer 0

#[test]
fn test_compile_literal_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_literal_byte() {
    let mut compiler = Compiler::new().bytes(true);
    let expr = Hir::new_literal(hir::Literal::Byte(b'a'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_literal_unicode_multiple() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_concat(vec![
        Hir::new_literal(hir::Literal::Unicode('a')),
        Hir::new_literal(hir::Literal::Unicode('b')),
    ]);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_literal_byte_multiple() {
    let mut compiler = Compiler::new().bytes(true);
    let expr = Hir::new_concat(vec![
        Hir::new_literal(hir::Literal::Byte(b'a')),
        Hir::new_literal(hir::Literal::Byte(b'b')),
    ]);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
fn test_compile_empty() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let result = compiler.c(&expr);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_compile_out_of_bounds_capture_index() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_group(hir::GroupKind::CaptureIndex(100));
    compiler.c(&expr).unwrap(); // This should panic due to invalid capture index.
}

