// Answer 0

#[test]
fn test_compile_empty_expression() {
    use syntax::hir::{Hir, HirKind};
    
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::Empty);
    
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.hole, Hole::None);
}

#[test]
fn test_compile_word_boundary_unicode() {
    use syntax::hir::{Hir, HirKind, WordBoundary};
    
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::WordBoundary(WordBoundary::Unicode));
    
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(compiler.compiled.has_unicode_word_boundary);
}

#[test]
fn test_compile_word_boundary_ascii() {
    use syntax::hir::{Hir, HirKind, WordBoundary};
    
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::WordBoundary(WordBoundary::Ascii));
    
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
}

#[test]
fn test_compile_word_boundary_unicode_negate() {
    use syntax::hir::{Hir, HirKind, WordBoundary};
    
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::WordBoundary(WordBoundary::UnicodeNegate));
    
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
    assert!(compiler.compiled.has_unicode_word_boundary);
}

#[test]
fn test_compile_word_boundary_ascii_negate() {
    use syntax::hir::{Hir, HirKind, WordBoundary};
    
    let mut compiler = Compiler::new();
    let expr = Hir::new(HirKind::WordBoundary(WordBoundary::AsciiNegate));
    
    let result = compiler.c(&expr);
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
}

