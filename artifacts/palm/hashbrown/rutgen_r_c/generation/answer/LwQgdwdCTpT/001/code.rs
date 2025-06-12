// Answer 0

#[test]
fn test_find_mut_existing_entry() {
    struct DummyAllocator;

    // Simulate a simple allocator
    impl Allocator for DummyAllocator {
        // Implement required allocator methods here
    }

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &i32| *val as u64; // Simple hasher function

    // Insert an entry into the HashTable.
    table.insert_unique(hasher(&1), 1, &hasher);

    // Attempt to mutate the existing entry
    if let Some(entry) = table.find_mut(hasher(&1), |val| *val == 1) {
        *entry += 1; // Mutate the entry
    }

    assert_eq!(table.find(hasher(&1), |val| *val == 2), Some(&2)); // Check if it was updated correctly
}

#[test]
fn test_find_mut_non_existent_entry() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement required allocator methods here
    }

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert an entry into the HashTable.
    table.insert_unique(hasher(&1), 1, &hasher);

    // Try to mutate a non-existent entry
    assert_eq!(table.find_mut(hasher(&2), |val| *val == 2), None);
}

#[test]
#[should_panic]
fn test_find_mut_panic() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement required allocator methods here
    }

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert an entry into the HashTable.
    table.insert_unique(hasher(&1), 1, &hasher);

    // Panic case: Mutate an entry using an equality function that should not match
    let _entry = table.find_mut(hasher(&1), |val| *val == 2).unwrap(); // This will panic
}

#[test]
fn test_find_mut_with_multiple_entries() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement required allocator methods here
    }

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &i32| *val as u64;

    // Insert multiple entries into the HashTable
    table.insert_unique(hasher(&1), 1, &hasher);
    table.insert_unique(hasher(&2), 2, &hasher);

    // Mutate the second entry
    if let Some(entry) = table.find_mut(hasher(&2), |val| *val == 2) {
        *entry += 2; // Update entry
    }

    assert_eq!(table.find(hasher(&2), |val| *val == 4), Some(&4)); // Check if it was updated correctly
}

#[test]
fn test_find_mut_empty_table() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement required allocator methods here
    }

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &i32| *val as u64;

    // Try to mutate an entry from an empty table
    assert_eq!(table.find_mut(hasher(&1), |val| *val == 1), None);
}

