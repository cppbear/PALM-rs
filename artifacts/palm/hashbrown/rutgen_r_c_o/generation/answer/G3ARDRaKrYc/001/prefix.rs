// Answer 0

#[test]
fn test_remove_entry_existing_element() {
    let alloc = Global;
    let mut table: RawTable<u64, _> = RawTable::with_capacity_in(10, alloc);
    
    let hash: u64 = 42;
    let value: u64 = 100;

    let _bucket = table.insert(hash, value, |x| *x);
    
    let result = table.remove_entry(hash, |_| true);
}

#[test]
fn test_remove_entry_existing_element_edge_case() {
    let alloc = Global;
    let mut table: RawTable<u64, _> = RawTable::with_capacity_in(10, alloc);
    
    let hash: u64 = u64::MAX;
    let value: u64 = 200;

    let _bucket = table.insert(hash, value, |x| *x);
    
    let result = table.remove_entry(hash, |_| true);
}

#[test]
fn test_remove_entry_multiple_elements() {
    let alloc = Global;
    let mut table: RawTable<u64, _> = RawTable::with_capacity_in(10, alloc);
    
    let hash1: u64 = 1;
    let value1: u64 = 300;

    let hash2: u64 = 2;
    let value2: u64 = 400;
    
    let _bucket1 = table.insert(hash1, value1, |x| *x);
    let _bucket2 = table.insert(hash2, value2, |x| *x);
    
    let result1 = table.remove_entry(hash1, |_| true);
    let result2 = table.remove_entry(hash2, |_| true);
}

#[test]
fn test_remove_entry_not_found() {
    let alloc = Global;
    let mut table: RawTable<u64, _> = RawTable::with_capacity_in(10, alloc);
    
    let hash: u64 = 3; // A hash that does not exist
    let result = table.remove_entry(hash, |_| true);
}

