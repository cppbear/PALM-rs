// Answer 0

#[test]
fn test_get_mut_valid_entry() {
    struct MockK;
    struct MockV {
        value: i32,
    }

    struct MockEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct MockIndices;

    struct MockIndexMapCore<K, V> {
        entries: MockEntries<K, V>,
    }

    impl<K, V> MockIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut {
                indices: &mut MockIndices,
                entries: &mut self.entries,
            }
        }
    }

    let mut map = MockIndexMapCore {
        entries: MockEntries {
            entries: vec![
                Bucket {
                    hash: HashValue::default(),
                    key: MockK,
                    value: MockV { value: 42 },
                },
            ],
        },
    };

    let mut indexed_entry = IndexedEntry::new(&mut map, 0);
    let value_ref: &mut MockV = indexed_entry.get_mut();
    value_ref.value = 99;

    assert_eq!(value_ref.value, 99);
}

#[should_panic]
fn test_get_mut_invalid_index() {
    struct MockK;
    struct MockV {
        value: i32,
    }

    struct MockEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct MockIndices;

    struct MockIndexMapCore<K, V> {
        entries: MockEntries<K, V>,
    }

    impl<K, V> MockIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut {
                indices: &mut MockIndices,
                entries: &mut self.entries,
            }
        }
    }

    let mut map = MockIndexMapCore {
        entries: MockEntries {
            entries: vec![
                Bucket {
                    hash: HashValue::default(),
                    key: MockK,
                    value: MockV { value: 42 },
                },
            ],
        },
    };

    let mut indexed_entry = IndexedEntry::new(&mut map, 1); // Invalid index
    indexed_entry.get_mut(); // This should panic
}

