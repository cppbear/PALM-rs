// Answer 0

#[test]
fn test_word_boundary_unicode() {
    let word_boundary = WordBoundary::Unicode;
    let result = Hir::word_boundary(word_boundary);
    
    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert!(!result.info.is_match_empty());
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
}

#[test]
fn test_word_boundary_unicode_negate() {
    let word_boundary = WordBoundary::UnicodeNegate;
    let result = Hir::word_boundary(word_boundary);
    
    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert!(result.info.is_match_empty());
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
}

#[test]
fn test_word_boundary_ascii() {
    let word_boundary = WordBoundary::Ascii;
    let result = Hir::word_boundary(word_boundary);
    
    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert!(!result.info.is_match_empty());
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
}

#[test]
fn test_word_boundary_ascii_negate() {
    let word_boundary = WordBoundary::AsciiNegate;
    let result = Hir::word_boundary(word_boundary);
    
    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert!(result.info.is_match_empty());
    assert!(!result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
}

