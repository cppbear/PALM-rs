// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required methods...
    }

    let alloc = TestAllocator;
    let table: HashTable<i32, TestAllocator> = HashTable::with_capacity_in(0, alloc);
    
    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required methods...
    }

    let alloc = TestAllocator;
    let table: HashTable<i32, TestAllocator> = HashTable::with_capacity_in(5, alloc);
    
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 5);
}

#[test]
fn test_with_capacity_in_high_capacity() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required methods...
    }

    let alloc = TestAllocator;
    let table: HashTable<i32, TestAllocator> = HashTable::with_capacity_in(1000, alloc);
    
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 1000);
}

#[test]
fn test_with_capacity_in_single_element() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required methods...
    }

    let alloc = TestAllocator;
    let table: HashTable<i32, TestAllocator> = HashTable::with_capacity_in(1, alloc);
    
    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 1);
}

