// Answer 0

#[test]
fn test_insert() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct DummyIndices;
    struct DummyEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> DummyEntries<K, V> {
        fn new() -> Self {
            DummyEntries { data: Vec::new() }
        }

        fn insert_unique(&mut self, _hash: HashValue, key: K, value: V) {
            self.data.push((key, value));
        }
    }

    let mut indices = DummyIndices;
    let mut entries = DummyEntries::new();
    let hash_builder = DummyHashBuilder;

    let ref_mut = RefMut {
        indices: &mut indices,
        entries: &mut entries,
    };

    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &hash_builder,
    };

    let key = String::from("key");
    let value = String::from("value");
    let (k_ref, v_ref) = raw_entry.insert(key.clone(), value.clone());

    assert_eq!(*k_ref, key);
    assert_eq!(*v_ref, value);
    assert_eq!(entries.data.len(), 1);
    assert_eq!(entries.data[0].0, key);
    assert_eq!(entries.data[0].1, value);
}

#[test]
fn test_insert_empty() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct DummyHashBuilder;

    impl BuildHasher for DummyHashBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct DummyIndices;
    struct DummyEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> DummyEntries<K, V> {
        fn new() -> Self {
            DummyEntries { data: Vec::new() }
        }

        fn insert_unique(&mut self, _hash: HashValue, key: K, value: V) {
            self.data.push((key, value));
        }
    }

    let mut indices = DummyIndices;
    let mut entries = DummyEntries::new();
    let hash_builder = DummyHashBuilder;

    let ref_mut = RefMut {
        indices: &mut indices,
        entries: &mut entries,
    };

    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &hash_builder,
    };

    let key = String::from("unique_key");
    let value = String::from("unique_value");
    let (k_ref, v_ref) = raw_entry.insert(key.clone(), value.clone());

    assert_eq!(*k_ref, key);
    assert_eq!(*v_ref, value);
    assert_eq!(entries.data.len(), 1);
    assert_eq!(entries.data[0].0, key);
    assert_eq!(entries.data[0].1, value);
}

