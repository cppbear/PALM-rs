// Answer 0

#[test]
fn test_word_boundary_unicode() {
    let boundary = WordBoundary::Unicode;
    let hir = Hir::word_boundary(boundary);
    assert_eq!(hir.kind, HirKind::WordBoundary(WordBoundary::Unicode));
    assert!(!hir.is_match_empty());
}

#[test]
fn test_word_boundary_unicode_negate() {
    let boundary = WordBoundary::UnicodeNegate;
    let hir = Hir::word_boundary(boundary);
    assert_eq!(hir.kind, HirKind::WordBoundary(WordBoundary::UnicodeNegate));
    assert!(hir.is_match_empty());
}

#[test]
fn test_word_boundary_ascii() {
    let boundary = WordBoundary::Ascii;
    let hir = Hir::word_boundary(boundary);
    assert_eq!(hir.kind, HirKind::WordBoundary(WordBoundary::Ascii));
    assert!(!hir.is_match_empty());
}

#[test]
fn test_word_boundary_ascii_negate() {
    let boundary = WordBoundary::AsciiNegate;
    let hir = Hir::word_boundary(boundary);
    assert_eq!(hir.kind, HirKind::WordBoundary(WordBoundary::AsciiNegate));
    assert!(hir.is_match_empty());
}

