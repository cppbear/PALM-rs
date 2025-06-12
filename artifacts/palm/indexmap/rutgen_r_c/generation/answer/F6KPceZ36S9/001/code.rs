// Answer 0

fn test_reserve_exact() {
    // Helper structure to initialize IndexMap since we don't have the implementation details.
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            self.hash_builder()
        }
    }

    // Test case 1: Reserve 0 additional values
    {
        let mut index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(10, TestHasher);
        let initial_capacity = index_set.capacity();
        index_set.reserve_exact(0);
        assert_eq!(index_set.capacity(), initial_capacity);
    }

    // Test case 2: Reserve additional values in a previously empty IndexSet
    {
        let mut index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(0, TestHasher);
        index_set.reserve_exact(5);
        assert!(index_set.capacity() >= 5);
    }

    // Test case 3: Reserve more values than the initial capacity
    {
        let mut index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(3, TestHasher);
        index_set.reserve_exact(10);
        assert!(index_set.capacity() >= 10);
    }

    // Test case 4: Reserve with a very high number, testing the limits
    {
        let mut index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(0, TestHasher);
        let large_additional = usize::MAX / 2; // Extreme test with a large number
        index_set.reserve_exact(large_additional);
        assert!(index_set.capacity() >= large_additional);
    }

    // Test case 5: Reserve exactly what is already allocated (no change)
    {
        let mut index_set: IndexSet<i32, TestHasher> = IndexSet::with_capacity_and_hasher(5, TestHasher);
        let initial_capacity = index_set.capacity();
        index_set.reserve_exact(initial_capacity);
        assert_eq!(index_set.capacity(), initial_capacity);
    }
}

