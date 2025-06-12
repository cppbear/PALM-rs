// Answer 0

#[test]
fn test_raw_table_inner_new() {
    struct Test;

    impl Test {
        const fn new() -> RawTableInner {
            RawTableInner::new()
        }
    }

    let raw_table = Test::new();
    assert_eq!(raw_table.bucket_mask, 0);
    assert_eq!(raw_table.items, 0);
    assert_eq!(raw_table.growth_left, 0);
    assert!(!raw_table.ctrl.is_null());
}

#[test]
fn test_raw_table_inner_ctrl_not_null() {
    struct Test;

    impl Test {
        const fn new() -> RawTableInner {
            RawTableInner::new()
        }
    }

    let raw_table = Test::new();
    assert!(!raw_table.ctrl.is_null());
}

