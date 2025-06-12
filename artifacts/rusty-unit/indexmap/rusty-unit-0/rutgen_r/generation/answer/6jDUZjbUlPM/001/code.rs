// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct IndexMap<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> IndexMap<K, V> {
    pub fn new() -> Self {
        IndexMap { entries: Vec::new() }
    }

    pub fn swap_remove_entry(&mut self) -> Option<Entry<K, V>> {
        self.entries.pop()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.push(Entry { key, value });
    }
}

impl<K, V> Entry<K, V> {
    pub fn swap_remove(self) -> V {
        let entry = self.swap_remove_entry().expect("The entry does not exist.");
        entry.value
    }
}

#[test]
fn test_swap_remove_empty_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    let entry = Entry { key: 0, value: "test".to_string() };
    
    // Attempting to swap remove from an empty map
    let result = std::panic::catch_unwind(|| entry.swap_remove());
    assert!(result.is_err());
}

#[test]
fn test_swap_remove_single_entry() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "value1".to_string());
    let entry = map.swap_remove_entry().unwrap();
    
    let value = entry.swap_remove();
    assert_eq!(value, "value1");
    assert!(map.entries.is_empty());
}

#[test]
fn test_swap_remove_multiple_entries() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());
    let entry = map.swap_remove_entry().unwrap();
    
    let value = entry.swap_remove();
    assert_eq!(value, "value2");
    assert_eq!(map.entries.len(), 1);
    
    let remaining_entry = map.swap_remove_entry().unwrap();
    assert_eq!(remaining_entry.value, "value1");
    assert!(map.entries.is_empty());
}

