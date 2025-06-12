// Answer 0

#[test]
fn test_swap_remove() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                indices: vec![],
                entries: vec![],
            }
        }

        fn borrow_mut(&mut self) -> RefMut<String, i32> {
            RefMut {
                indices: &mut self.indices,
                entries: &mut self.entries,
            }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.indices.push(self.entries.len());
            self.entries.push((key, value));
        }

        fn swap_remove_index(&mut self, index: usize) -> Option<(String, i32)> {
            if index < self.entries.len() {
                let last_index = self.entries.len() - 1;
                self.indices[index] = last_index; // Update the index map
                let last_entry = self.entries.swap_remove(index);
                Some(last_entry)
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    map.insert("key3".to_string(), 3);

    let index = 1;
    let mut entry = IndexedEntry::new(&mut map.borrow_mut(), index);
    let value = entry.swap_remove();

    assert_eq!(value, 2);
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0], ("key1".to_string(), 1));
    assert_eq!(map.entries[1], ("key3".to_string(), 3));
}

