// Answer 0

#[test]
fn test_allocation_size_empty() {
    let table: RawTable<i32> = RawTable::new_in(Global);
    assert_eq!(table.allocation_size(), 0);
}

#[test]
fn test_allocation_size_non_empty() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(8, Global);
    unsafe {
        // Assuming that we can allocate some memory and that the implementation properly
        // uses this capacity.
        for i in 0..8 {
            let _bucket = table.insert(i as u64, i, |&x| x);
        }
    }
    assert!(table.allocation_size() > 0);
}

#[test]
fn test_allocation_size_after_clear() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(8, Global);
    unsafe {
        for i in 0..8 {
            let _bucket = table.insert(i as u64, i, |&x| x);
        }
    }
    assert!(table.allocation_size() > 0);
    table.clear();
    assert!(table.allocation_size() > 0);
}

