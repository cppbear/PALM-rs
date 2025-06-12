// Answer 0

#[derive(Debug)]
struct HashValue(usize);

#[derive(Debug, Eq, PartialEq)]
struct Key(String);

#[derive(Debug)]
struct Value(String);

struct Entry<K, V> {
    key: K,
    value: V,
}

struct Indices {
    map: std::collections::HashMap<usize, usize>,
}

impl Indices {
    fn entry(&mut self, key: usize) -> EntryAction {
        if self.map.contains_key(&key) {
            EntryAction::Occupied(self.map.get(&key).unwrap())
        } else {
            EntryAction::Vacant { key }
        }
    }

    fn insert(&mut self, key: usize, value: usize) {
        self.map.insert(key, value);
    }
}

enum EntryAction<'a> {
    Occupied(&'a usize),
    Vacant { key: usize },
}

struct HashTable<K, V> {
    entries: Vec<Entry<K, V>>,
    indices: Indices,
}

impl<K, V> HashTable<K, V>
where
    K: Eq,
{
    fn push_entry(&mut self, _hash: HashValue, key: K, value: V) {
        self.entries.push(Entry { key, value });
    }
}

fn equivalent<K>(key: &K, _entries: &[Entry<K, Value>]) -> usize {
    0 // Simulating an equivalency function returning a hash
}

fn get_hash(_entries: &[Entry<Key, Value>]) -> usize {
    0 // Simulating a hash retrieval that returns 0
}

impl<K, V> HashTable<K, V>
where
    K: Eq,
{
    pub(crate) fn replace_full(
        &mut self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> (usize, Option<(K, V)>) {
        let eq = equivalent(&key, &self.entries);
        let hasher = get_hash(&self.entries);
        match self.indices.entry(hash.0) {
            EntryAction::Occupied(&entry_index) => {
                let entry = &mut self.entries[entry_index];
                let kv = (
                    std::mem::replace(&mut entry.key, key),
                    std::mem::replace(&mut entry.value, value),
                );
                (entry_index, Some(kv))
            }
            EntryAction::Vacant { key } => {
                let index = self.entries.len();
                self.indices.insert(hash.0, index);
                self.push_entry(hash, key, value);
                (index, None)
            }
        }
    }
}

#[test]
fn test_replace_full_occupied() {
    let mut hash_table = HashTable {
        entries: vec![
            Entry { key: Key("key1".to_string()), value: Value("value1".to_string()) },
        ],
        indices: Indices {
            map: std::collections::HashMap::new(),
        },
    };

    hash_table.indices.insert(0, 0); // Simulating entry being occupied

    let (index, old_kv) = hash_table.replace_full(HashValue(0), Key("key1".to_string()), Value("new_value1".to_string()));

    assert_eq!(index, 0);
    assert_eq!(old_kv, Some((Key("key1".to_string()), Value("value1".to_string()))));
}

#[test]
fn test_replace_full_vacant() {
    let mut hash_table = HashTable {
        entries: vec![],
        indices: Indices {
            map: std::collections::HashMap::new(),
        },
    };

    let (index, old_kv) = hash_table.replace_full(HashValue(1), Key("key2".to_string()), Value("value2".to_string()));

    assert_eq!(index, 0);
    assert_eq!(old_kv, None);
}

