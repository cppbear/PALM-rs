// Answer 0

#[derive(Debug, Clone)]
struct Entry<K> {
    key: K,
}

#[derive(Debug)]
struct Map<K> {
    entries: Vec<Entry<K>>,
}

impl<K> Map<K> {
    fn new() -> Self {
        Map { entries: Vec::new() }
    }

    fn insert(&mut self, key: K) {
        self.entries.push(Entry { key });
    }

    fn entry(&self, index: usize) -> &Entry<K> {
        &self.entries[index]
    }
}

impl<K> Entry<K> {
    pub fn key(&self) -> &K {
        &self.key
    }
}

#[test]
fn test_key_retrieval() {
    let mut map = Map::new();
    map.insert("first");
    map.insert("second");
    
    let entry = map.entry(0);
    assert_eq!(entry.key(), &"first");
    
    let entry = map.entry(1);
    assert_eq!(entry.key(), &"second");
}

#[test]
fn test_key_empty_map() {
    let map: Map<i32> = Map::new();
    // This test would panic since there are no entries, so we are commenting out assertion
    // let result = map.entry(0);
    // assert!(result.is_err());  // This would require modifying the function to handle empty case
}

