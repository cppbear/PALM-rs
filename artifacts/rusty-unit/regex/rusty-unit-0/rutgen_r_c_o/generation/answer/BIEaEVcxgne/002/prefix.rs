// Answer 0

#[test]
fn test_is_empty_true() {
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo::default(),
    };
    hir.is_empty();
}

