// Answer 0

#[derive(Debug)]
struct Entry<K> {
    key: K,
}

struct Map<K> {
    entries: Vec<Entry<K>>,
    index: usize,
}

impl<K> Map<K> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: 0,
        }
    }

    pub fn insert(&mut self, key: K) {
        self.entries.push(Entry { key });
        self.index = self.entries.len() - 1; // Point to the last entry
    }

    pub fn key(&self) -> &K {
        &self.entries[self.index()].key
    }

    /// This function safely gets the current index and will panic if out of bounds.
    fn index(&self) -> usize {
        if self.index >= self.entries.len() {
            panic!("Index out of bounds");
        }
        self.index
    }
}

#[test]
fn test_key_single_element() {
    let mut map = Map::new();
    map.insert("key1");
    assert_eq!(map.key(), &"key1");
}

#[test]
fn test_key_multiple_elements() {
    let mut map = Map::new();
    map.insert("key1");
    map.insert("key2");
    assert_eq!(map.key(), &"key2");
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_key_empty_map() {
    let map: Map<String> = Map::new();
    map.key(); // This should panic since there are no entries
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_key_after_clear() {
    let mut map = Map::new();
    map.insert("key1");
    map.insert("key2");
    map.index = 0; // Manually setting index to point to first entry
    map.entries.clear(); // Clear entries
    map.key(); // This should panic since entries are cleared
}

