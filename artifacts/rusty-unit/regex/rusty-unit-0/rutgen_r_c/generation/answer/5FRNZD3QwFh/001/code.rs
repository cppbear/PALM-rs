// Answer 0

#[test]
fn test_hir_info_new() {
    let hir_info = HirInfo::new();
    assert_eq!(hir_info, HirInfo { bools: 0 });
}

