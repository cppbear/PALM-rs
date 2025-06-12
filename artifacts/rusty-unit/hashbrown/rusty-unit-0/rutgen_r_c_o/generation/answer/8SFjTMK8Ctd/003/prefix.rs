// Answer 0

#[test]
fn test_erase_with_index_zero_on_empty_bucket() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 4);
    unsafe { table.erase(0) }
}

#[test]
fn test_erase_with_index_at_bucket_mask() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 4);
    unsafe { table.erase(table.bucket_mask) }
}

#[test]
fn test_erase_below_bucket_mask() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 4);
    unsafe { table.erase(table.bucket_mask - 1) }
}

#[test]
fn test_erase_above_empty_control_so_it_does_not_panic() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut table = RawTableInner::with_capacity(&alloc, table_layout, 8);
    unsafe { table.erase(3) }
}

