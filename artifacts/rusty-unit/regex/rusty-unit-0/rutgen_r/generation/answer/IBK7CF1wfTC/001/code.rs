// Answer 0

#[test]
fn test_word_boundary_ascii_negate() {
    // Given a WordBoundary::AsciiNegate to test the word_boundary function
    let word_boundary = WordBoundary::AsciiNegate;
    
    // When we call the function
    let result = word_boundary(word_boundary);
    
    // Then we expect the return type to be Hir
    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert!(!result.info.always_utf8()); // ASCII negated should allow invalid UTF-8
    assert!(result.info.match_empty()); // Is negated, should match empty string
}

#[test]
fn test_word_boundary_normal() {
    // Given a WordBoundary::Normal to test the word_boundary function
    let word_boundary = WordBoundary::Normal;
    
    // When we call the function
    let result = word_boundary(word_boundary);
    
    // Then we expect the return type to be Hir
    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert!(result.info.always_utf8()); // Should always be UTF-8
    assert!(!result.info.match_empty()); // Normal should not match empty string
}

