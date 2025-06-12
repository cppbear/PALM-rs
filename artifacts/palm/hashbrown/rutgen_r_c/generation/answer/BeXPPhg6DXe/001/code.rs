// Answer 0

#[test]
fn test_allocation_size_empty() {
    let table: RawTable<u8> = RawTable::new_in(Global);
    assert_eq!(table.allocation_size(), 0);
}

#[test]
fn test_allocation_size_non_empty() {
    let capacity = 8;
    let table: RawTable<u8> = RawTable::with_capacity_in(capacity, Global);
    let size = table.allocation_size();
    assert!(size > 0);
}

#[test]
fn test_allocation_size_large_capacity() {
    let capacity = 1024;
    let table: RawTable<u8> = RawTable::with_capacity_in(capacity, Global);
    let size = table.allocation_size();
    assert!(size > 0);
}

#[test]
fn test_allocation_size_after_multiple_insertions() {
    let mut table: RawTable<u8> = RawTable::new_in(Global);
    for i in 0..10 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let size = table.allocation_size();
    assert!(size > 0);
}

