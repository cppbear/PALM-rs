// Answer 0

#[test]
fn test_drain_iter_from_valid_iterator() {
    let allocator = Global;
    let mut table: RawTable<i32> = RawTable::new_in(allocator);

    // Insert elements into the table
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
    }

    // Get an iterator from the table
    let iter = unsafe { table.iter() };

    // Drain from the iterator
    let mut drain = unsafe { table.drain_iter_from(iter) };

    // Verify that the drained elements count matches the table elements
    assert_eq!(drain.iter.len(), 2);
    assert_eq!(table.len(), 0);
}

#[test]
#[should_panic(expected = "assertion failed: iter.len() == self.len()")]
fn test_drain_iter_from_invalid_iterator_length() {
    let allocator = Global;
    let mut table: RawTable<i32> = RawTable::new_in(allocator);

    // Insert elements into the table
    unsafe {
        table.insert(3, 30, |x| *x);
        table.insert(4, 40, |x| *x);
    }

    // Create an iterator that does not match the length of the table
    let iter = unsafe { table.iter() };

    // Modify the table to change its length
    unsafe {
        table.clear();
    }

    // Try to drain from the iterator, which should panic
    let _ = unsafe { table.drain_iter_from(iter) };
}

