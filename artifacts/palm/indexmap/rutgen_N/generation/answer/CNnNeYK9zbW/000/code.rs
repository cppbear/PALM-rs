// Answer 0

#[derive(Hash, Eq, PartialEq)]
struct KeyType {
    id: usize,
}

struct ValueType {
    data: String,
}

struct Entry<K, V> {
    key: K,
    value: V,
}

struct IndexMap<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> IndexMap<K, V> {
    fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: Hash + Eq + ?Sized,
        K: Hash + Eq,
    {
        self.entries.iter().position(|entry| &entry.key == key)
    }

    fn as_entries_mut(&mut self) -> &mut [Entry<K, V>] {
        &mut self.entries
    }
}

impl<K: Hash + Eq, V> IndexMap<K, V> {
    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of(key) {
            let entry = &mut self.as_entries_mut()[i];
            Some((i, &entry.key, &mut entry.value))
        } else {
            None
        }
    }
}

#[test]
fn test_get_full_mut_existing_key() {
    let mut map = IndexMap {
        entries: vec![
            Entry {
                key: KeyType { id: 1 },
                value: ValueType { data: "value1".into() },
            },
            Entry {
                key: KeyType { id: 2 },
                value: ValueType { data: "value2".into() },
            },
        ],
    };

    let key = KeyType { id: 1 };
    let result = map.get_full_mut(&key).unwrap();

    assert_eq!(result.0, 0);
    assert_eq!(result.1.id, 1);
    result.2.data = "updated_value".into();
    assert_eq!(map.entries[0].value.data, "updated_value");
}

#[test]
fn test_get_full_mut_non_existing_key() {
    let mut map = IndexMap {
        entries: vec![
            Entry {
                key: KeyType { id: 1 },
                value: ValueType { data: "value1".into() },
            },
        ],
    };

    let key = KeyType { id: 3 };
    let result = map.get_full_mut(&key);
    assert!(result.is_none());
}

