// Answer 0

#[test]
fn test_drain_iter_from_valid_case() {
    let allocator = Global {};
    let mut raw_table = RawTable::with_capacity_in(10, allocator);
    let mut iter = unsafe { raw_table.iter() };

    raw_table.insert(1, "one", |v| *v);
    raw_table.insert(2, "two", |v| *v);
    
    let drain = unsafe { raw_table.drain_iter_from(iter) };
}

#[test]
fn test_drain_iter_from_edge_case_length() {
    let allocator = Global {};
    let mut raw_table = RawTable::with_capacity_in(256, allocator);
    let mut iter = unsafe { raw_table.iter() };

    for i in 0..256 {
        raw_table.insert(i as u64, i.to_string(), |v| *v);
    }

    let drain = unsafe { raw_table.drain_iter_from(iter) };
}

#[test]
fn test_drain_iter_from_small_case() {
    let allocator = Global {};
    let mut raw_table = RawTable::with_capacity_in(5, allocator);
    let mut iter = unsafe { raw_table.iter() };

    raw_table.insert(5, "five", |v| *v);
    
    let drain = unsafe { raw_table.drain_iter_from(iter) };
}

#[test]
#[should_panic]
fn test_drain_iter_from_invalid_case() {
    let allocator = Global {};
    let mut raw_table = RawTable::with_capacity_in(3, allocator);
    let mut iter = unsafe { raw_table.iter() };

    raw_table.insert(1, "one", |v| *v);
    
    // Attempting to drain with an iter that does not cover all items
    let drain = unsafe { raw_table.drain_iter_from(iter) };
}

