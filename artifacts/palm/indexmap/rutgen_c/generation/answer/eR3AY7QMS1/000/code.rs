// Answer 0

#[test]
fn test_shift_remove() {
    struct DummyIndices;
    struct DummyEntries<K, V> {
        data: Vec<(K, V)>,
    }

    struct DummyIndexMapCore<K, V> {
        indices: DummyIndices,
        entries: DummyEntries<K, V>,
    }

    impl<K, V> DummyIndexMapCore<K, V> {
        fn new() -> Self {
            Self {
                indices: DummyIndices,
                entries: DummyEntries { data: Vec::new() },
            }
        }
        
        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut {
                indices: &mut self.indices,
                entries: &mut self.entries,
            }
        }
        
        fn push(&mut self, key: K, value: V) {
            self.entries.data.push((key, value));
        }
        
        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.data.len() {
                let removed_entry = self.entries.data.remove(index);
                Some(removed_entry)
            } else {
                None
            }
        }
    }

    impl<'a, K, V> IndexedEntry<'a, K, V> {
        fn new(map: &'a mut DummyIndexMapCore<K, V>, index: usize) -> Self {
            Self {
                map: map.borrow_mut(),
                index,
            }
        }
    }

    let mut map = DummyIndexMapCore::new();
    map.push("key1", "value1");
    map.push("key2", "value2");
    map.push("key3", "value3");

    let entry = IndexedEntry::new(&mut map, 1); // Entry at index 1 ("key2", "value2")
    let value = entry.shift_remove();

    assert_eq!(value, "value2");
    assert_eq!(map.entries.data.len(), 2);
    assert_eq!(map.entries.data[0].1, "value1");
    assert_eq!(map.entries.data[1].1, "value3");
}

#[test]
fn test_shift_remove_boundary() {
    struct DummyIndices;
    struct DummyEntries<K, V> {
        data: Vec<(K, V)>,
    }

    struct DummyIndexMapCore<K, V> {
        indices: DummyIndices,
        entries: DummyEntries<K, V>,
    }

    impl<K, V> DummyIndexMapCore<K, V> {
        fn new() -> Self {
            Self {
                indices: DummyIndices,
                entries: DummyEntries { data: Vec::new() },
            }
        }

        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut {
                indices: &mut self.indices,
                entries: &mut self.entries,
            }
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.data.push((key, value));
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.data.len() {
                let removed_entry = self.entries.data.remove(index);
                Some(removed_entry)
            } else {
                None
            }
        }
    }

    impl<'a, K, V> IndexedEntry<'a, K, V> {
        fn new(map: &'a mut DummyIndexMapCore<K, V>, index: usize) -> Self {
            Self {
                map: map.borrow_mut(),
                index,
            }
        }
    }

    let mut map = DummyIndexMapCore::new();
    map.push("key1", "value1");

    let entry = IndexedEntry::new(&mut map, 0); // Entry at index 0 ("key1", "value1")
    let value = entry.shift_remove();

    assert_eq!(value, "value1");
    assert_eq!(map.entries.data.len(), 0); // Should be empty after removal
}

#[test]
#[should_panic]
fn test_shift_remove_out_of_bounds() {
    struct DummyIndices;
    struct DummyEntries<K, V> {
        data: Vec<(K, V)>,
    }

    struct DummyIndexMapCore<K, V> {
        indices: DummyIndices,
        entries: DummyEntries<K, V>,
    }

    impl<K, V> DummyIndexMapCore<K, V> {
        fn new() -> Self {
            Self {
                indices: DummyIndices,
                entries: DummyEntries { data: Vec::new() },
            }
        }

        fn borrow_mut(&mut self) -> RefMut<K, V> {
            RefMut {
                indices: &mut self.indices,
                entries: &mut self.entries,
            }
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.data.push((key, value));
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.data.len() {
                let removed_entry = self.entries.data.remove(index);
                Some(removed_entry)
            } else {
                None
            }
        }
    }

    impl<'a, K, V> IndexedEntry<'a, K, V> {
        fn new(map: &'a mut DummyIndexMapCore<K, V>, index: usize) -> Self {
            Self {
                map: map.borrow_mut(),
                index,
            }
        }
    }

    let mut map = DummyIndexMapCore::new();
    map.push("key1", "value1");

    let entry = IndexedEntry::new(&mut map, 1); // Invalid index
    entry.shift_remove(); // This should panic
}

