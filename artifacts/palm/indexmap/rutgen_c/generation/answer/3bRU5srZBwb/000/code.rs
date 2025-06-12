// Answer 0

#[test]
fn test_get_index_mut2_existing_index() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: vec![Bucket { hash: 1, key: 1, value: "one".to_string() }],
        },
        hash_builder: TestHasher,
    };

    if let Some((key, value)) = map.get_index_mut2(0) {
        assert_eq!(*key, 1);
        assert_eq!(*value, "one");
    } else {
        panic!("Expected to find an entry at index 0");
    }
}

#[test]
fn test_get_index_mut2_out_of_bounds() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: vec![Bucket { hash: 1, key: 1, value: "one".to_string() }],
        },
        hash_builder: TestHasher,
    };

    let result = map.get_index_mut2(1);
    assert!(result.is_none(), "Expected None for out-of-bounds index");
}

