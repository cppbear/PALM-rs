// Answer 0

#[test]
fn test_empty_hir() {
    let empty_hir = Hir::empty();
    assert_eq!(empty_hir.kind(), &HirKind::Empty);
    assert!(empty_hir.is_always_utf8());
    assert!(empty_hir.is_all_assertions());
    assert!(!empty_hir.is_anchored_start());
    assert!(!empty_hir.is_anchored_end());
    assert!(!empty_hir.is_any_anchored_start());
    assert!(!empty_hir.is_any_anchored_end());
    assert!(empty_hir.is_match_empty());
}

