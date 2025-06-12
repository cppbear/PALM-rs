// Answer 0

#[test]
fn test_anchor_start_text() {
    use crate::Hir;
    use crate::HirKind;
    use crate::Anchor;
    
    let anchor = Anchor::StartText;
    let result = Hir::anchor(anchor);
    
    assert_eq!(result.kind, HirKind::Anchor(anchor));
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
    assert!(result.info.is_match_empty());
}

#[test]
fn test_anchor_end_text() {
    use crate::Hir;
    use crate::HirKind;
    use crate::Anchor;
    
    let anchor = Anchor::EndText;
    let result = Hir::anchor(anchor);
    
    assert_eq!(result.kind, HirKind::Anchor(anchor));
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(!result.info.is_anchored_start());
    assert!(result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(result.info.is_any_anchored_end());
    assert!(result.info.is_match_empty());
}

