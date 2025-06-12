// Answer 0

#[test]
fn test_as_entries() {
    #[derive(Clone, Debug)]
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, MockHasher> = IndexMap {
        core: IndexMapCore {
            indices: Vec::new(),
            entries: Vec::new(),
        },
        hash_builder: MockHasher,
    };

    index_map.core.entries.push(Bucket {
        hash: HashValue::default(),
        key: 1,
        value: "one".to_string(),
    });

    index_map.core.entries.push(Bucket {
        hash: HashValue::default(),
        key: 2,
        value: "two".to_string(),
    });

    let entries = index_map.as_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "one");
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[1].value, "two");
}

#[test]
fn test_as_entries_empty() {
    #[derive(Clone, Debug)]
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let index_map: IndexMap<i32, String, MockHasher> = IndexMap {
        core: IndexMapCore {
            indices: Vec::new(),
            entries: Vec::new(),
        },
        hash_builder: MockHasher,
    };

    let entries = index_map.as_entries();
    assert_eq!(entries.len(), 0);
}

