// Answer 0

#[test]
fn test_allocation_size_or_zero_empty_singleton() {
    let table_layout = TableLayout { size: 0, ctrl_align: 0 };
    let raw_table_inner = RawTableInner {
        bucket_mask: 0,
        ctrl: NonNull::dangling(),
        growth_left: 0,
        items: 0,
    };
    unsafe {
        let size = raw_table_inner.allocation_size_or_zero(table_layout);
    }
}

#[test]
fn test_allocation_size_or_zero_with_non_empty_table() {
    let table_layout = TableLayout { size: 64, ctrl_align: 8 };
    let raw_table_inner = RawTableInner {
        bucket_mask: 1,
        ctrl: NonNull::new(0 as *mut u8).unwrap(),
        growth_left: 1,
        items: 0,
    };
    unsafe {
        let size = raw_table_inner.allocation_size_or_zero(table_layout);
    }
}

