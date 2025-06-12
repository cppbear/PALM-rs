// Answer 0

#[test]
fn test_shrink_to_fit() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {}

    let mut table = HashTable::with_capacity_in(100, DummyAllocator);
    let hasher = |val: &u64| *val; // Simple identity hasher for the test
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    
    assert!(table.capacity() >= 100);
    table.shrink_to_fit(hasher);
    assert!(table.capacity() >= 2);
    assert_eq!(table.len(), 2);
}

#[test]
fn test_shrink_to_fit_empty() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {}

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &u64| *val; // Simple identity hasher for the test
    
    assert!(table.capacity() == 0);
    table.shrink_to_fit(hasher);
    assert!(table.capacity() == 0);
}

#[test]
fn test_shrink_to_fit_single_entry() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {}

    let mut table = HashTable::with_capacity_in(5, DummyAllocator);
    let hasher = |val: &u64| *val; // Simple identity hasher for the test
    table.insert_unique(hasher(&1), 1, hasher);
    
    assert!(table.capacity() >= 5);
    table.shrink_to_fit(hasher);
    assert!(table.capacity() >= 1);
    assert_eq!(table.len(), 1);
}

