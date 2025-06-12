// Answer 0

#[test]
fn test_raw_table_len_empty() {
    let table: RawTable<u8> = RawTable::new_in(Global);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_raw_table_len_non_empty() {
    let mut table: RawTable<u8> = RawTable::with_capacity_in(4, Global);
    // Simulating adding elements - assuming there's an `insert` method appropriately available
    for i in 1..=3 {
        unsafe {
            table.insert(i as u64, i, |&x| x);
        }
    }
    assert_eq!(table.len(), 3);
}

#[test]
fn test_raw_table_len_after_clear() {
    let mut table: RawTable<u8> = RawTable::with_capacity_in(4, Global);
    // Simulating adding elements - assuming there's an `insert` method appropriately available
    for i in 1..=2 {
        unsafe {
            table.insert(i as u64, i, |&x| x);
        }
    }
    table.clear();
    assert_eq!(table.len(), 0);
}

