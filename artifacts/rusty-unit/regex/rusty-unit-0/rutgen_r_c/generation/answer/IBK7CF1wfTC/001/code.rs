// Answer 0

#[test]
fn test_word_boundary_unicode() {
    let word_boundary_unicode = WordBoundary::Unicode;
    let result = Hir::word_boundary(word_boundary_unicode);
    assert_eq!(result.kind, HirKind::WordBoundary(WordBoundary::Unicode));
    assert!(!result.info.is_match_empty());
}

#[test]
fn test_word_boundary_unicode_negate() {
    let word_boundary_unicode_negate = WordBoundary::UnicodeNegate;
    let result = Hir::word_boundary(word_boundary_unicode_negate);
    assert_eq!(result.kind, HirKind::WordBoundary(WordBoundary::UnicodeNegate));
    assert!(result.info.is_match_empty());
}

#[test]
fn test_word_boundary_ascii() {
    let word_boundary_ascii = WordBoundary::Ascii;
    let result = Hir::word_boundary(word_boundary_ascii);
    assert_eq!(result.kind, HirKind::WordBoundary(WordBoundary::Ascii));
    assert!(!result.info.is_match_empty());
}

#[test]
fn test_word_boundary_ascii_negate() {
    let word_boundary_ascii_negate = WordBoundary::AsciiNegate;
    let result = Hir::word_boundary(word_boundary_ascii_negate);
    assert_eq!(result.kind, HirKind::WordBoundary(WordBoundary::AsciiNegate));
    assert!(result.info.is_match_empty());
    assert!(!result.info.is_always_utf8());
}

