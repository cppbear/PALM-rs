// Answer 0

#[test]
fn test_retain2_removes_entries_based_on_condition() {
    struct TestHashBuilder;
    
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain2(|k, v| *k != 2);

    let entries: Vec<_> = map.iter_mut2().collect();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].0, &1);
    assert_eq!(entries[0].1, "one");
    assert_eq!(entries[1].0, &3);
    assert_eq!(entries[1].1, "three");
}

#[test]
fn test_retain2_keeps_all_when_condition_is_true() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain2(|_k, _v| true);

    let entries: Vec<_> = map.iter_mut2().collect();
    assert_eq!(entries.len(), 3);
}

#[test]
fn test_retain2_removes_all_when_condition_is_false() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };
    
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    map.retain2(|_k, _v| false);

    let entries: Vec<_> = map.iter_mut2().collect();
    assert_eq!(entries.len(), 0);
}

