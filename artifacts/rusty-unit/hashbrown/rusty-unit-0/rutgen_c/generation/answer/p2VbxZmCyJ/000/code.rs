// Answer 0

#[test]
fn test_clear_empty_table() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement Allocator methods as required.
    }
    
    let mut table = HashTable::new_in(TestAllocator);
    table.clear();
    assert!(table.is_empty());
}

#[test]
fn test_clear_with_elements() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement Allocator methods as required.
    }

    let mut table = HashTable::with_capacity_in(10, TestAllocator);
    let hasher = |val: &_| val as u64; // simple identity hash for the test
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.clear();
    assert!(table.is_empty());
}

#[test]
fn test_clear_non_empty_table_then_reinsert() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement Allocator methods as required.
    }

    let mut table = HashTable::with_capacity_in(10, TestAllocator);
    let hasher = |val: &_| val as u64; // simple identity hash for the test
    table.insert_unique(hasher(&1), 1, hasher);
    table.clear();
    assert!(table.is_empty());
    table.insert_unique(hasher(&3), 3, hasher);
    assert_eq!(table.len(), 1);
    assert!(!table.is_empty());
}

