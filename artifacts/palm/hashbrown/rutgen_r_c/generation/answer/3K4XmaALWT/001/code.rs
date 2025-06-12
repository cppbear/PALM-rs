// Answer 0

#[test]
fn test_allocation_size() {
    use crate::raw::Global; // Assuming Global is a proper Allocator
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required allocator methods here for the test as needed
    }

    let allocator = TestAllocator;
    
    // Test case 1: Create a new empty hash table and check allocation size
    let table_empty = HashTable::new_in(allocator);
    assert_eq!(table_empty.allocation_size(), 0); // Expecting empty allocation size

    // Test case 2: Create a hash table with a capacity and check allocation size
    let capacity = 10;
    let table_with_capacity = HashTable::with_capacity_in(capacity, allocator);
    assert!(table_with_capacity.allocation_size() > 0); // Expecting non-zero allocation size

    // Test case 3: Insert an item and check if allocation size increases
    let mut table = HashTable::new_in(allocator);
    table.insert_unique(1, "Hello", |s: &str| s.len() as u64); // Insert item
    assert!(table.allocation_size() > 0); // Expecting non-zero allocation size

    // Test case 4: Clear the table and check allocation size
    table.clear();
    assert!(table.allocation_size() > 0); // Expecting allocation size unchanged after clear

    // Test case 5: Allocate again with larger capacity and check allocation changes
    let table_large = HashTable::with_capacity_in(100, allocator);
    assert!(table_large.allocation_size() > table_with_capacity.allocation_size()); // Check if larger allocated
}

