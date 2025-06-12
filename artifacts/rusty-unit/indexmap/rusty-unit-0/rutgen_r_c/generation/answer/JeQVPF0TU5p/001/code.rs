// Answer 0

#[test]
fn test_get_present_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    // Insert values to the IndexSet
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());

    // Test getting a value that is present
    assert_eq!(index_set.get(&1), Some(&1));
    assert_eq!(index_set.get(&2), Some(&2));
    assert_eq!(index_set.get(&3), Some(&3));
}

#[test]
fn test_get_absent_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    // Insert some values into the IndexSet
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    // Test getting a value that is not present
    assert_eq!(index_set.get(&3), None);
}

#[test]
fn test_get_with_different_type() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    // Insert a value
    index_set.map.insert(1, ());

    // Test getting a value with different type
    let result: Option<&i32> = index_set.get(&(1 as i32));
    assert_eq!(result, Some(&1));
}

