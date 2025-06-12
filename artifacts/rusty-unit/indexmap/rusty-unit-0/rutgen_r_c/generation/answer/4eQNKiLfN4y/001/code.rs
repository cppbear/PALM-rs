// Answer 0

#[test]
fn test_capacity_empty() {
    struct TestHashBuilder;
    
    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }
    
    let map: IndexMap<u32, u32, TestHashBuilder> = IndexMap::with_hasher(TestHashBuilder::default());
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_with_length() {
    struct TestHashBuilder;

    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }
    
    let mut map: IndexMap<u32, u32, TestHashBuilder> = IndexMap::with_capacity_and_hasher(5, TestHashBuilder::default());
    assert!(map.capacity() >= 5);
}

#[test]
fn test_capacity_after_inserts() {
    struct TestHashBuilder;

    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let mut map: IndexMap<u32, u32, TestHashBuilder> = IndexMap::with_capacity_and_hasher(2, TestHashBuilder::default());
    map.insert(1, 10);
    map.insert(2, 20);
    assert!(map.capacity() >= 2);
}

#[test]
fn test_capacity_with_reserve() {
    struct TestHashBuilder;

    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let mut map: IndexMap<u32, u32, TestHashBuilder> = IndexMap::with_hasher(TestHashBuilder::default());
    map.reserve(10);
    assert!(map.capacity() >= 10);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_capacity_after_truncation() {
    struct TestHashBuilder;

    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }
    
    let mut map: IndexMap<u32, u32, TestHashBuilder> = IndexMap::with_capacity_and_hasher(5, TestHashBuilder::default());
    map.truncate(2);
    assert!(map.capacity() >= 2);
}

#[test]
fn test_capacity_large_insertions() {
    struct TestHashBuilder;

    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let mut map: IndexMap<u32, u32, TestHashBuilder> = IndexMap::with_capacity_and_hasher(usize::MAX, TestHashBuilder::default());
    assert_eq!(map.capacity(), usize::MAX);
}

