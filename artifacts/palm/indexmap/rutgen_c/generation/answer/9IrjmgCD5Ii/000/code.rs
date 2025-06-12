// Answer 0

#[test]
fn test_shift_remove_entry_removes_correct_entry() {
    struct MockIndices {
        len: usize,
    }
    
    struct MockEntries<K, V> {
        data: Vec<(K, V)>,
    }

    struct MockIndexMapCore<K, V> {
        indices: MockIndices,
        entries: MockEntries<K, V>,
    }
    
    impl<K, V> MockIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }
    
    impl<K: Clone, V> RefMut<'_, K, V> {
        fn new(indices: &mut MockIndices, entries: &mut MockEntries<K, V>) -> Self {
            Self { indices, entries }
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.data.len() {
                let entry = self.entries.data.remove(index);
                self.indices.len -= 1;
                Some(entry)
            } else {
                None
            }
        }
    }

    let mut map_core = MockIndexMapCore {
        indices: MockIndices { len: 3 },
        entries: MockEntries {
            data: vec![(1, "a"), (2, "b"), (3, "c")],
        },
    };

    let index_entry = IndexedEntry::new(&mut map_core, 1);
    let (key, value) = index_entry.shift_remove_entry();
    
    assert_eq!(key, 2);
    assert_eq!(value, "b");
    assert_eq!(map_core.entries.data.len(), 2);
    assert_eq!(map_core.entries.data[0], (1, "a"));
    assert_eq!(map_core.entries.data[1], (3, "c"));
}

#[test]
#[should_panic]
fn test_shift_remove_entry_panics_on_invalid_index() {
    struct MockIndices {
        len: usize,
    }
    
    struct MockEntries<K, V> {
        data: Vec<(K, V)>,
    }

    struct MockIndexMapCore<K, V> {
        indices: MockIndices,
        entries: MockEntries<K, V>,
    }

    impl<K, V> MockIndexMapCore<K, V> {
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut::new(&mut self.indices, &mut self.entries)
        }
    }

    impl<K: Clone, V> RefMut<'_, K, V> {
        fn new(indices: &mut MockIndices, entries: &mut MockEntries<K, V>) -> Self {
            Self { indices, entries }
        }
        
        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.data.len() {
                let entry = self.entries.data.remove(index);
                self.indices.len -= 1;
                Some(entry)
            } else {
                None
            }
        }
    }

    let mut map_core = MockIndexMapCore {
        indices: MockIndices { len: 3 },
        entries: MockEntries {
            data: vec![(1, "a"), (2, "b"), (3, "c")],
        },
    };

    let index_entry = IndexedEntry::new(&mut map_core, 5);
    index_entry.shift_remove_entry();
}

