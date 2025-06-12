// Answer 0

#[derive(Debug)]
struct MapEntry {
    key: String,
    value: String,
}

struct IndexMap {
    entries: Vec<MapEntry>,
}

impl IndexMap {
    fn new() -> Self {
        IndexMap { entries: Vec::new() }
    }

    fn as_entries(&self) -> &[MapEntry] {
        &self.entries
    }

    fn insert(&mut self, key: String, value: String) {
        self.entries.push(MapEntry { key, value });
    }
}

#[test]
fn test_as_entries_empty() {
    let map = IndexMap::new();
    let entries = map.as_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_as_entries_single_entry() {
    let mut map = IndexMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    let entries = map.as_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, "key1");
    assert_eq!(entries[0].value, "value1");
}

#[test]
fn test_as_entries_multiple_entries() {
    let mut map = IndexMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    map.insert("key3".to_string(), "value3".to_string());
    let entries = map.as_entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[1].key, "key2");
    assert_eq!(entries[2].key, "key3");
}

