// Answer 0

#[test]
fn test_compile_empty() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_kind(HirKind::Empty);
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_literal_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_kind(HirKind::Literal(hir::Literal::Unicode('a')));
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_literal_byte() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = true; // To allow byte compilation
    let expr = Hir::from_kind(HirKind::Literal(hir::Literal::Byte(0x61))); // 'a'
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_class_unicode() {
    let mut compiler = Compiler::new();
    let unicode_class = hir::Class::Unicode(hir::UnicodeClass::new(vec![(0x0041, 0x005A)])); // A-Z
    let expr = Hir::from_kind(HirKind::Class(unicode_class));
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_class_bytes() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = true; // To allow byte class compilation
    let byte_class = hir::Class::Bytes(hir::BytesClass::new(vec![(b'A', b'Z')])); // A-Z
    let expr = Hir::from_kind(HirKind::Class(byte_class));
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_anchor_start_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_kind(HirKind::Anchor(hir::Anchor::StartLine));
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_anchor_end_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_kind(HirKind::Anchor(hir::Anchor::EndLine));
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_anchor_start_text() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_kind(HirKind::Anchor(hir::Anchor::StartText));
    assert!(compiler.c(&expr).is_ok());
}

#[test]
fn test_compile_anchor_end_text() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_kind(HirKind::Anchor(hir::Anchor::EndText));
    assert!(compiler.c(&expr).is_ok());
}

