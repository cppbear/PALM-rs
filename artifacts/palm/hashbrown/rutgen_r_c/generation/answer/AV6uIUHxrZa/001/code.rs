// Answer 0

#[test]
fn test_remove_existing_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: HashSet<i32, TestHasher> = HashSet {
        map: HashMap {
            hash_builder: TestHasher,
            table: RawTable::new(),
        },
    };

    set.insert(2);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);
}

#[test]
fn test_remove_non_existing_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: HashSet<i32, TestHasher> = HashSet {
        map: HashMap {
            hash_builder: TestHasher,
            table: RawTable::new(),
        },
    };

    assert_eq!(set.remove(&3), false);
}

#[test]
fn test_remove_with_different_borrowed_form() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: HashSet<i32, TestHasher> = HashSet {
        map: HashMap {
            hash_builder: TestHasher,
            table: RawTable::new(),
        },
    };

    set.insert(4);
    let borrowed_value: &i32 = &4;
    assert_eq!(set.remove(borrowed_value), true);
    assert_eq!(set.remove(borrowed_value), false);
}

