// Answer 0

#[test]
fn test_with_capacity_small() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let capacity = 1; // Minimum valid capacity
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
}

#[test]
fn test_with_capacity_medium() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let capacity = 1024; // Valid medium capacity
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
}

#[test]
fn test_with_capacity_large() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let capacity = 1_000_000; // Large capacity
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
}

#[test]
#[should_panic] // Expected to panic when exceeding isize::MAX
fn test_with_capacity_exceed_max() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let capacity = isize::MAX as usize + 1; // Capacity exceeding isize::MAX
    let _table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
}

