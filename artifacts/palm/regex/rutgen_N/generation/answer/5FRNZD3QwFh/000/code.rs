// Answer 0

#[test]
fn test_new_hir_info() {
    struct HirInfo {
        bools: u32,
    }

    fn new() -> HirInfo {
        HirInfo {
            bools: 0,
        }
    }

    let hir_info = new();
    assert_eq!(hir_info.bools, 0);
}

