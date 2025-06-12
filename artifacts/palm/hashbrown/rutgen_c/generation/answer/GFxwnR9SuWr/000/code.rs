// Answer 0

#[test]
fn test_insert_unique() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary methods for the allocator
    }

    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert an element
    let occupied_entry = table.insert_unique(hasher(&1), 1, hasher);
    assert_eq!(occupied_entry.hash, hasher(&1));
}

#[test]
fn test_insert_multiple_elements() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary methods for the allocator
    }

    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert multiple elements
    let occupied_entry1 = table.insert_unique(hasher(&1), 1, hasher);
    let occupied_entry2 = table.insert_unique(hasher(&2), 2, hasher);
    
    assert_eq!(occupied_entry1.hash, hasher(&1));
    assert_eq!(occupied_entry2.hash, hasher(&2));
}

#[test]
fn test_insert_with_same_hash() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary methods for the allocator
    }

    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert an element
    let occupied_entry1 = table.insert_unique(hasher(&1), 1, hasher);
    // Insert another element with the same hash
    let occupied_entry2 = table.insert_unique(hasher(&1), 2, hasher);

    assert_eq!(occupied_entry1.hash, hasher(&1));
    assert_eq!(occupied_entry2.hash, hasher(&1)); // Same hash for both
}

#[test]
fn test_insert_empty_table() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary methods for the allocator
    }

    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert an element into an empty table
    let occupied_entry = table.insert_unique(hasher(&10), 10, hasher);
    assert_eq!(occupied_entry.hash, hasher(&10));
}

