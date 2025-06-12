// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.entry, 0);
    assert_eq!(patch.hole, Hole::None);
}

#[test]
fn test_compile_unicode_literal() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(patch.entry > 0);
    assert_eq!(patch.hole, Hole::None);
}

#[test]
fn test_compile_byte_literal() {
    let mut compiler = Compiler::new().bytes(true);
    let expr = Hir::new_literal(hir::Literal::Byte(b'a'));
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(patch.entry > 0);
    assert_eq!(patch.hole, Hole::None);
}

#[test]
fn test_compile_unicode_class_all_ascii() {
    let mut compiler = Compiler::new();
    let cls = hir::Class::Unicode(hir::UnicodeClass::new_all_ascii());
    let expr = Hir::new_class(cls);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(patch.entry > 0);
    assert_eq!(patch.hole, Hole::None);
}

#[test]
fn test_compile_bytes_class_empty() {
    let mut compiler = Compiler::new().bytes(true);
    let cls = hir::Class::Bytes(hir::BytesClass::new_empty());
    let expr = Hir::new_class(cls);
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(patch.entry > 0);
    assert_eq!(patch.hole, Hole::None);
}

#[test]
fn test_compile_bytes_class_with_no_iter() {
    let mut compiler = Compiler::new().bytes(true);
    let cls = hir::Class::Bytes(hir::BytesClass::from_ranges(vec![]));
    let expr = Hir::new_class(cls);
    let result = compiler.c(&expr);
    assert!(result.is_err());
}

