// Answer 0

#[test]
fn test_insert_full_vacant_entry() {
    struct HashValue {
        value: usize,
    }

    impl HashValue {
        fn get(&self) -> usize {
            self.value
        }
    }

    struct Entry {
        value: String,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry>,
        indices: HashMap<usize, usize>,
    }

    impl<K: Eq + Copy, V> MyMap<K, V> {
        pub(crate) fn insert_full(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>) {
            let eq = false; // simulate that equivalent returns false
            let hasher = hash.get();  // Simulate hash value
            match self.indices.entry(hash.get()) {
                Entry::Occupied(entry) => {
                    let i = *entry.get();
                    (i, Some(std::mem::replace(&mut self.entries[i].value, value)))
                }
                Entry::Vacant(entry) => {
                    let i = self.entries.len();
                    entry.insert(i);
                    self.entries.push(Entry { value: value.to_string() }); // store value as String for simplicity
                    (i, None)
                }
            }
        }
    }

    use std::collections::HashMap;

    let mut my_map = MyMap {
        entries: Vec::new(),
        indices: HashMap::new(),
    };

    let hash = HashValue { value: 1 };
    let key = 'a';
    let value = "value1";

    let (index, returned_value) = my_map.insert_full(hash, key, value);

    assert_eq!(index, 0);
    assert!(returned_value.is_none());
    assert_eq!(my_map.entries.len(), 1);
    assert_eq!(my_map.entries[0].value, "value1");
}

