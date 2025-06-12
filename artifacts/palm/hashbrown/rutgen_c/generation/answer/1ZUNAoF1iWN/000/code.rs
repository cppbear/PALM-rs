// Answer 0

#[test]
fn test_shrink_to() {
    struct DummyAllocator;
    struct DummyHashTable {
        raw: RawTable<u64, DummyAllocator>,
    }
    
    impl DummyAllocator {
        pub fn new() -> Self {
            DummyAllocator {}
        }
    }
    
    let mut table = HashTable::with_capacity_in(100, DummyAllocator::new());
    let hasher = |val: &u64| *val; // Simple identity hasher for u64
    
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    
    assert!(table.capacity() >= 100);
    
    table.shrink_to(10, hasher);
    assert!(table.capacity() >= 10);
    
    table.shrink_to(0, hasher);
    assert!(table.capacity() >= 2);
}

#[should_panic]
fn test_shrink_to_panics_on_min_capacity_too_large() {
    struct DummyAllocator;
    let mut table = HashTable::with_capacity_in(20, DummyAllocator);
    
    let hasher = |val: &u64| *val;
    
    table.shrink_to(30, hasher); // Should panic since the current capacity is less than 30
}

#[test]
fn test_shrink_to_with_exact_capacity() {
    struct DummyAllocator;
    let mut table = HashTable::with_capacity_in(50, DummyAllocator);
    
    let hasher = |val: &u64| *val;
    
    table.insert_unique(hasher(&1), 1, hasher);
    assert!(table.capacity() >= 50);
    
    table.shrink_to(50, hasher);
    assert_eq!(table.capacity(), 50); // Should remain the same
}

#[test]
fn test_shrink_to_with_zero_capacity() {
    struct DummyAllocator;
    let mut table = HashTable::with_capacity_in(100, DummyAllocator);
    
    let hasher = |val: &u64| *val;
    
    table.insert_unique(hasher(&1), 1, hasher);
    
    assert!(table.capacity() >= 100);
    
    table.shrink_to(0, hasher);
    assert!(table.capacity() >= 2); // Should allow more than zero
}

