// Answer 0

#[test]
fn test_find_existing_entry() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);
    
    assert_eq!(table.find(hasher(&2), |&val| val == 2), Some(&2));
}

#[test]
fn test_find_non_existing_entry() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    
    assert_eq!(table.find(hasher(&4), |&val| val == 4), None);
}

#[test]
fn test_find_entry_with_custom_condition() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(hasher(&5), 5, hasher);
    table.insert_unique(hasher(&10), 10, hasher);
    
    assert_eq!(table.find(hasher(&5), |&val| val % 5 == 0), Some(&5));
    assert_eq!(table.find(hasher(&10), |&val| val % 3 == 0), None);
}

