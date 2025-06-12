// Answer 0

#[test]
fn test_with_capacity_and_hasher_in_non_zero_capacity() {
    struct DefaultHashBuilder;

    impl Default for DefaultHashBuilder {
        fn default() -> Self {
            DefaultHashBuilder
        }
    }

    struct SimpleAllocator;

    impl SimpleAllocator {
        fn new() -> Self {
            SimpleAllocator
        }
    }

    let capacity = 10;
    let hash_builder = DefaultHashBuilder::default();
    let allocator = SimpleAllocator::new();
    
    let map = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, allocator);
    
    assert!(map.table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_and_hasher_in_zero_capacity() {
    struct DefaultHashBuilder;

    impl Default for DefaultHashBuilder {
        fn default() -> Self {
            DefaultHashBuilder
        }
    }

    struct SimpleAllocator;

    impl SimpleAllocator {
        fn new() -> Self {
            SimpleAllocator
        }
    }

    let capacity = 0;
    let hash_builder = DefaultHashBuilder::default();
    let allocator = SimpleAllocator::new();
    
    let map = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, allocator);
    
    assert_eq!(map.table.capacity(), 0);
}

