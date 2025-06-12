// Answer 0

#[test]
fn test_insert_sorted_new_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    let result = index_set.insert_sorted(10);
    assert_eq!(result, (0, true));
    let result = index_set.insert_sorted(20);
    assert_eq!(result, (1, true));
    let result = index_set.insert_sorted(15);
    assert_eq!(result, (1, true));
    let result = index_set.insert_sorted(10);
    assert_eq!(result, (0, false));
}

#[test]
fn test_insert_sorted_with_negative_and_zero() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    let result = index_set.insert_sorted(0);
    assert_eq!(result, (0, true));
    let result = index_set.insert_sorted(-5);
    assert_eq!(result, (0, true));
    let result = index_set.insert_sorted(-1);
    assert_eq!(result, (1, true));
    let result = index_set.insert_sorted(-5);
    assert_eq!(result, (0, false));
}

#[test]
fn test_insert_sorted_boundary_conditions() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: TestHasher,
        },
    };

    let max_value = i32::MAX;
    let min_value = i32::MIN;

    let result = index_set.insert_sorted(min_value);
    assert_eq!(result, (0, true));
    let result = index_set.insert_sorted(max_value);
    assert_eq!(result, (1, true));
    let result = index_set.insert_sorted(max_value);
    assert_eq!(result, (1, false));
    let result = index_set.insert_sorted(min_value);
    assert_eq!(result, (0, false));
}

