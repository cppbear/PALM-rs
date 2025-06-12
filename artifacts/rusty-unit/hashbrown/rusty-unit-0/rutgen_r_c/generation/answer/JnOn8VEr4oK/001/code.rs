// Answer 0

#[test]
fn test_allocator_with_global_allocator() {
    use crate::raw::Global;

    // Initialize a HashTable with a global allocator
    let table: HashTable<i32, Global> = HashTable::new_in(Global);

    // Assert that the allocator is the same as the one we initialized
    assert_eq!(table.allocator(), &Global);
}

#[test]
fn test_allocator_with_custom_allocator() {
    use crate::raw::{Allocator, Global};
    
    struct CustomAllocator; // Placeholder for a theoretical custom allocator
    impl Allocator for CustomAllocator {
        // Implement required methods...
    }

    // Initialize a HashTable with a custom allocator
    let custom_alloc = CustomAllocator;
    let table: HashTable<i32, CustomAllocator> = HashTable::new_in(custom_alloc);

    // Assert that the allocator is the same as the one we initialized
    assert_eq!(table.allocator(), &table.raw.allocator());
}

#[test]
fn test_allocator_on_empty_table() {
    use crate::raw::Global;

    // Create an empty HashTable
    let table: HashTable<u32, Global> = HashTable::new_in(Global);

    // Assert that the allocator is Global
    assert_eq!(table.allocator(), &Global);
}

