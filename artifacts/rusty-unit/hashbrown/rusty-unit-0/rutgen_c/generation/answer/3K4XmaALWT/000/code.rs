// Answer 0

#[test]
fn test_allocation_size() {
    use crate::raw::Global;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement the required methods for the Allocator trait as needed.
    }

    // Create a new HashTable instance with the test allocator.
    let alloc = TestAllocator;
    let table = HashTable::new_in(alloc);

    // Assert that the initial allocation size is as expected (usually 0 or some lower size).
    assert_eq!(table.allocation_size(), 0);
}

#[test]
fn test_allocation_size_with_capacity() {
    use crate::raw::Global;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement the required methods for the Allocator trait as needed.
    }

    // Create a new HashTable with a specified capacity.
    let alloc = TestAllocator;
    let capacity = 10;
    let mut table = HashTable::with_capacity_in(capacity, alloc);

    // After insertion or reserve operation, validate the allocation size.
    table.insert_unique(1, "item1", |s| *s as u64);
    assert!(table.allocation_size() > 0);
}

#[test]
fn test_allocation_size_after_clear() {
    use crate::raw::Global;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement the required methods for the Allocator trait as needed.
    }

    // Create a HashTable and manipulate it.
    let alloc = TestAllocator;
    let mut table = HashTable::new_in(alloc);

    table.insert_unique(1, "item1", |s| *s as u64);
    assert!(table.allocation_size() > 0);
    
    // Clear the HashTable and check the allocation size.
    table.clear();
    assert_eq!(table.allocation_size(), 0); // Expect 0 or the minimum size after clearing.
}

