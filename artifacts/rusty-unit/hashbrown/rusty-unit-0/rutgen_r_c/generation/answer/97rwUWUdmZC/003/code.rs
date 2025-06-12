// Answer 0

#[test]
fn test_calculate_layout_for_success() {
    struct TestStruct;
    impl TestStruct {
        fn get_table_layout() -> TableLayout {
            TableLayout::new::<TestStruct>()
        }
    }

    let layout = TestStruct::get_table_layout();
    let buckets = 8; // 8 is a power of two
    let result = layout.calculate_layout_for(buckets);
    assert!(result.is_some());
}

#[test]
fn test_calculate_layout_for_checked_mul_overflow() {
    struct TestStruct;
    impl TestStruct {
        fn get_table_layout() -> TableLayout {
            TableLayout::new::<TestStruct>()
        }
    }

    let layout = TestStruct::get_table_layout();
    let buckets = usize::MAX / layout.size + 1; // causes checked_mul to return None
    let result = layout.calculate_layout_for(buckets);
    assert!(result.is_none());
}

#[test]
fn test_calculate_layout_for_checked_add_overflow_ctrl_offset() {
    struct TestStruct;
    impl TestStruct {
        fn get_table_layout() -> TableLayout {
            TableLayout::new::<TestStruct>()
        }
    }

    let layout = TestStruct::get_table_layout();
    let buckets = 8; // 8 is a power of two
    let ctrl_offset = layout.size.checked_mul(buckets).unwrap() + layout.ctrl_align - 1;

    // Set the layout size to cause checked_add to overflow
    let bad_buckets = isize::MAX as usize - (ctrl_offset + 1); // forces overflow on addition
    let result = layout.calculate_layout_for(bad_buckets);
    assert!(result.is_none());
}

#[test]
fn test_calculate_layout_for_checked_add_overflow_len() {
    struct TestStruct;
    impl TestStruct {
        fn get_table_layout() -> TableLayout {
            TableLayout::new::<TestStruct>()
        }
    }

    let layout = TestStruct::get_table_layout();
    let buckets = 8; // 8 is a power of two
    let ctrl_offset = layout.size.checked_mul(buckets).unwrap() + layout.ctrl_align - 1;

    // Set the length to be too large
    const INVALID_LENGTH: usize = isize::MAX as usize; // set len to something that will cause failure
    let bad_buckets = INVALID_LENGTH - ctrl_offset - Group::WIDTH; // forces overflow on addition
    let result = layout.calculate_layout_for(bad_buckets);
    assert!(result.is_none());
}

