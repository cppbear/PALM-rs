// Answer 0

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let hasher = TestHasher;
    let map = IndexMap::<i32, i32, TestHasher>::with_capacity_and_hasher(10, hasher);
    assert_eq!(map.capacity(), 10);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity_large() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let hasher = TestHasher;
    let map = IndexMap::<i32, i32, TestHasher>::with_capacity_and_hasher(1_000_000, hasher);
    assert_eq!(map.capacity(), 1_000_000);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_and_hasher_non_zero_capacity_edge_case() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let hasher = TestHasher;
    let map = IndexMap::<i32, i32, TestHasher>::with_capacity_and_hasher(usize::MAX, hasher);
    assert!(map.capacity() > 0); // Ensure that the map has some capacity
}

#[test]
fn test_with_capacity_and_hasher_minimal_non_zero() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = TestHasher;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    let hasher = TestHasher;
    let map = IndexMap::<i32, i32, TestHasher>::with_capacity_and_hasher(1, hasher);
    assert_eq!(map.capacity(), 1);
    assert!(map.is_empty());
}

