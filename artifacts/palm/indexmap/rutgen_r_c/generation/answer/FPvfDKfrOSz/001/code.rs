// Answer 0

#[test]
fn test_as_entries_empty() {
    struct TestHashBuilder;
    
    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder::default(),
    };

    let entries: &[Bucket<i32, i32>] = index_map.as_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_as_entries_single() {
    struct TestHashBuilder;
    
    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let mut index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder::default(),
    };

    index_map.insert(1, 10);
  
    let entries: &[Bucket<i32, i32>] = index_map.as_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, 10);
}

#[test]
fn test_as_entries_multiple() {
    struct TestHashBuilder;
    
    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let mut index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder::default(),
    };

    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);

    let entries: &[Bucket<i32, i32>] = index_map.as_entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[2].key, 3);
}

#[test]
fn test_as_entries_with_retraining() {
    struct TestHashBuilder;

    impl Default for TestHashBuilder {
        fn default() -> Self {
            TestHashBuilder
        }
    }

    let mut index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder::default(),
    };

    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.retain(|&mut k, &mut v| k != 1);

    let entries: &[Bucket<i32, i32>] = index_map.as_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 2);
    assert_eq!(entries[0].value, 20);
}

