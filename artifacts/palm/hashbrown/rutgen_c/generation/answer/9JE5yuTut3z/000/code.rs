// Answer 0

#[test]
fn test_is_empty_initially() {
    struct TestAllocator; // Placeholder for the Allocator trait
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let table: HashTable<i32, TestAllocator> = HashTable::new_in(allocator);

    assert!(table.is_empty());
}

#[test]
fn test_is_empty_after_insertion() {
    struct TestAllocator; // Placeholder for the Allocator trait
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(allocator);
    
    let hasher = |val: &i32| *val as u64;

    table.insert_unique(hasher(&1), 1, hasher);
    assert!(!table.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    struct TestAllocator; // Placeholder for the Allocator trait
    impl Allocator for TestAllocator {}

    let allocator = TestAllocator;
    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(allocator);
    
    let hasher = |val: &i32| *val as u64;

    table.insert_unique(hasher(&1), 1, hasher);
    assert!(!table.is_empty());

    table.clear();
    assert!(table.is_empty());
}

