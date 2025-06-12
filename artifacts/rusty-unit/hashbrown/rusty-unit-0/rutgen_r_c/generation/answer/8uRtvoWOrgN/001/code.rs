// Answer 0

#[test]
fn test_get_many_mut_unique_keys() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut raw_table: RawTable<u64, MockAllocator> = RawTable::new_in(MockAllocator);

    // Populate the table with unique keys.
    // Assuming a hypothetical insert method exists that inserts key-value pairs.
    raw_table.insert(1, 100, |value| *value);
    raw_table.insert(2, 200, |value| *value);
    raw_table.insert(3, 300, |value| *value);
    raw_table.insert(4, 400, |value| *value);

    // Attempt to get mutable references to the entries with unique keys.
    let hashes = [1, 2, 3];
    let result = raw_table.get_many_mut(hashes, |index, key| {
        match index {
            0 => *key == 100,
            1 => *key == 200,
            2 => *key == 300,
            _ => false,
        }
    });

    // Ensure we received the expected mutable references.
    assert!(result[0].is_some());
    assert!(result[1].is_some());
    assert!(result[2].is_some());
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_duplicate_keys() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut raw_table: RawTable<u64, MockAllocator> = RawTable::new_in(MockAllocator);

    // Populate the table with unique keys.
    raw_table.insert(1, 100, |value| *value);
    raw_table.insert(2, 200, |value| *value);
    raw_table.insert(3, 300, |value| *value);

    // Attempt to get mutable references to the entries with a duplicate key.
    let hashes = [1, 2, 1]; // 1 is duplicated here.
    let _result = raw_table.get_many_mut(hashes, |index, key| {
        match index {
            0 => *key == 100,
            1 => *key == 200,
            2 => *key == 100, // Still evaluates to true for the duplicate.
            _ => false,
        }
    });
}

