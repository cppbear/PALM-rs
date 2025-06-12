// Answer 0

#[test]
fn test_empty_hir() {
    let hir = Hir::empty();
    assert_eq!(hir.kind, HirKind::Empty);
    assert!(hir.is_always_utf8());
    assert!(hir.is_all_assertions());
    assert!(!hir.is_anchored_start());
    assert!(!hir.is_anchored_end());
    assert!(!hir.is_any_anchored_start());
    assert!(!hir.is_any_anchored_end());
    assert!(hir.is_match_empty());
}

