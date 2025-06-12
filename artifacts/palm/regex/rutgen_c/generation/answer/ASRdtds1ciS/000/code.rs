// Answer 0

#[test]
fn test_anchor_start_text() {
    let anchor_expression = Hir::anchor(Anchor::StartText);
    assert_eq!(anchor_expression.kind, HirKind::Anchor(Anchor::StartText));
    assert!(anchor_expression.is_always_utf8());
    assert!(anchor_expression.is_all_assertions());
    assert!(anchor_expression.is_anchored_start());
    assert!(!anchor_expression.is_anchored_end());
    assert!(anchor_expression.is_match_empty());
}

#[test]
fn test_anchor_end_text() {
    let anchor_expression = Hir::anchor(Anchor::EndText);
    assert_eq!(anchor_expression.kind, HirKind::Anchor(Anchor::EndText));
    assert!(anchor_expression.is_always_utf8());
    assert!(anchor_expression.is_all_assertions());
    assert!(!anchor_expression.is_anchored_start());
    assert!(anchor_expression.is_anchored_end());
    assert!(anchor_expression.is_match_empty());
}

#[test]
fn test_anchor_start_line() {
    let anchor_expression = Hir::anchor(Anchor::StartLine);
    assert_eq!(anchor_expression.kind, HirKind::Anchor(Anchor::StartLine));
    assert!(anchor_expression.is_always_utf8());
    assert!(anchor_expression.is_all_assertions());
    assert!(!anchor_expression.is_anchored_start());
    assert!(!anchor_expression.is_anchored_end());
    assert!(anchor_expression.is_match_empty());
}

#[test]
fn test_anchor_end_line() {
    let anchor_expression = Hir::anchor(Anchor::EndLine);
    assert_eq!(anchor_expression.kind, HirKind::Anchor(Anchor::EndLine));
    assert!(anchor_expression.is_always_utf8());
    assert!(anchor_expression.is_all_assertions());
    assert!(!anchor_expression.is_anchored_start());
    assert!(!anchor_expression.is_anchored_end());
    assert!(anchor_expression.is_match_empty());
}

