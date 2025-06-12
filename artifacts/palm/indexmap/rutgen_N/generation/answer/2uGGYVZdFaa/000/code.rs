// Answer 0

#[test]
fn test_shift_remove_entry() {
    struct MockEntry<K, V> {
        index: Vec<(K, V)>,
    }

    impl<K, V> MockEntry<K, V> {
        pub fn new() -> Self {
            Self {
                index: Vec::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.index.push((key, value));
        }

        pub fn remove(&mut self) -> Option<(usize, (K, V))> {
            if self.index.is_empty() {
                return None;
            }
            let index = self.index.len() - 1;
            Some((index, self.index.remove(index)))
        }
    }

    struct RefMut<'a, K, V> {
        table: &'a mut Vec<(K, V)>,
        entries: usize,
    }

    impl<'a, K, V> RefMut<'a, K, V> {
        pub fn new(table: &'a mut Vec<(K, V)>, entries: usize) -> Self {
            Self { table, entries }
        }

        pub fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            let (key, value) = self.table.remove(index);
            (key, value)
        }
    }

    let mut entry = MockEntry::new();
    entry.insert(1, "a");
    entry.insert(2, "b");
    entry.insert(3, "c");

    let (index, item) = entry.remove().unwrap();
    let mut ref_mut = RefMut::new(&mut entry.index, index);
    let (key, value) = ref_mut.shift_remove_finish(index);
    
    assert_eq!(key, 3);
    assert_eq!(value, "c");
    assert_eq!(entry.index.len(), 2);
    assert_eq!(entry.index[0], (1, "a"));
    assert_eq!(entry.index[1], (2, "b"));
}

