// Answer 0

#[test]
fn test_get_index_mut2_valid_index() {
    struct TestBuildHasher;
    
    impl BuildHasher for TestBuildHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestBuildHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::default(),
            entries: Entries::from(vec![
                Bucket { hash: 1.into(), key: 1, value: "one".to_string() },
                Bucket { hash: 2.into(), key: 2, value: "two".to_string() },
            ]),
        },
        hash_builder: TestBuildHasher,
    };

    let result = map.get_index_mut2(0);
    assert!(result.is_some());
    if let Some((key, value)) = result {
        assert_eq!(*key, 1);
        assert_eq!(*value, "one");
    }
}

#[test]
fn test_get_index_mut2_out_of_bounds_index() {
    struct TestBuildHasher;
    
    impl BuildHasher for TestBuildHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestBuildHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::default(),
            entries: Entries::from(vec![
                Bucket { hash: 1.into(), key: 1, value: "one".to_string() },
            ]),
        },
        hash_builder: TestBuildHasher,
    };

    let result = map.get_index_mut2(2);
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_get_index_mut2_negative_index() {
    struct TestBuildHasher;
    
    impl BuildHasher for TestBuildHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestBuildHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::default(),
            entries: Entries::from(vec![
                Bucket { hash: 1.into(), key: 1, value: "one".to_string() },
            ]),
        },
        hash_builder: TestBuildHasher,
    };

    let _ = map.get_index_mut2(-1);
}

