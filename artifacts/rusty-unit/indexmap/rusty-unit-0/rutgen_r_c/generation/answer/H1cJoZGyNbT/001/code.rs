// Answer 0

#[test]
fn test_key_valid_index() {
    struct MockEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct MockIndices {
        len: usize,
    }

    struct MockIndexMapCore<K, V> {
        entries: MockEntries<K, V>,
        indices: MockIndices,
    }

    impl<K, V> RefMut<'_, K, V> {
        fn new(entries: &mut MockEntries<K, V>, indices: &mut MockIndices) -> Self {
            Self {
                entries,
                indices,
            }
        }
    }

    let mut entries = MockEntries {
        entries: vec![
            Bucket { hash: 0.into(), key: "Key1", value: "Value1" },
            Bucket { hash: 0.into(), key: "Key2", value: "Value2" },
        ],
    };

    let mut indices = MockIndices { len: 2 };
    let mut map = MockIndexMapCore { entries, indices };
    let indexed_entry = IndexedEntry::new(&mut map, 1);
    
    assert_eq!(indexed_entry.key(), &"Key2");
}

#[test]
#[should_panic]
fn test_key_invalid_index_too_large() {
    struct MockEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct MockIndices {
        len: usize,
    }

    struct MockIndexMapCore<K, V> {
        entries: MockEntries<K, V>,
        indices: MockIndices,
    }

    impl<K, V> RefMut<'_, K, V> {
        fn new(entries: &mut MockEntries<K, V>, indices: &mut MockIndices) -> Self {
            Self {
                entries,
                indices,
            }
        }
    }

    let mut entries = MockEntries {
        entries: vec![
            Bucket { hash: 0.into(), key: "Key1", value: "Value1" },
        ],
    };

    let mut indices = MockIndices { len: 1 };
    let mut map = MockIndexMapCore { entries, indices };
    let indexed_entry = IndexedEntry::new(&mut map, 1);  // index is out of bounds
    let _key = indexed_entry.key();
}

#[test]
#[should_panic]
fn test_key_invalid_index_negative() {
    struct MockEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct MockIndices {
        len: usize,
    }

    struct MockIndexMapCore<K, V> {
        entries: MockEntries<K, V>,
        indices: MockIndices,
    }

    impl<K, V> RefMut<'_, K, V> {
        fn new(entries: &mut MockEntries<K, V>, indices: &mut MockIndices) -> Self {
            Self {
                entries,
                indices,
            }
        }
    }

    let mut entries = MockEntries {
        entries: vec![
            Bucket { hash: 0.into(), key: "Key1", value: "Value1" },
        ],
    };

    let mut indices = MockIndices { len: 1 };
    let mut map = MockIndexMapCore { entries, indices };
    let indexed_entry = IndexedEntry::new(&mut map, usize::MAX);  // index is out of bounds
    let _key = indexed_entry.key();
}

