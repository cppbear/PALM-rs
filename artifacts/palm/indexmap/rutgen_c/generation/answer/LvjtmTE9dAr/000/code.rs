// Answer 0

#[test]
fn test_key_mut() {
    struct TestHashBuilder;
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
    }

    struct TestIndexMapCore<K, V> {
        inner: TestMap<K, V>,
    }

    impl<K, V> TestIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> &mut TestMap<K, V> {
            &mut self.inner
        }
    }

    let mut test_map = TestMap::new();
    test_map.entries.push((String::from("key"), 42));

    let mut index_map_core = TestIndexMapCore { inner: test_map };
    let mut indexed_entry = IndexedEntry::new(&mut index_map_core, 0);

    let key_mut = indexed_entry.key_mut();
    *key_mut = String::from("new_key");

    assert_eq!(indexed_entry.key(), &String::from("new_key"));
}

