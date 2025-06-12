// Answer 0

#[test]
fn test_allocator_with_default_allocator() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}
    
    let table: HashTable<u32, TestAllocator> = HashTable::new_in(TestAllocator);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_custom_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let table: HashTable<u32, TestAllocator> = HashTable::with_capacity_in(1024, TestAllocator);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_when_empty() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let table: HashTable<u32, TestAllocator> = HashTable::new_in(TestAllocator);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_large_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let table: HashTable<u32, TestAllocator> = HashTable::with_capacity_in(1 << 30, TestAllocator);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_hash_function() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<u32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hash = 123456789;
    let allocator = table.allocator();
}

