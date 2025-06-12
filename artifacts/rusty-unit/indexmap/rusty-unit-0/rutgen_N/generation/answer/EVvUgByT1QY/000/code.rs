// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct IndexMap<K, V> {
    entries: Vec<Bucket<K, V>>,
}

impl<K, V> IndexMap<K, V> {
    pub fn new() -> Self {
        IndexMap {
            entries: Vec::new(),
        }
    }

    pub fn as_entries_mut(&mut self) -> &mut Vec<Bucket<K, V>> {
        &mut self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn push(&mut self, key: K, value: V) {
        self.entries.push(Bucket { key, value });
    }
}

impl<K, V> IndexMap<K, V> {
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.as_entries_mut().get_mut(index).map(|bucket| (&bucket.key, &mut bucket.value))
    }
}

#[test]
fn test_get_index_mut_valid_index() {
    let mut map = IndexMap::new();
    map.push("key1", "value1");
    map.push("key2", "value2");

    let result = map.get_index_mut(1);
    assert!(result.is_some());
    let (key, value) = result.unwrap();
    assert_eq!(*key, "key2");
    *value = "updated_value";
    assert_eq!(map.get_index_mut(1).unwrap().1, "updated_value");
}

#[test]
fn test_get_index_mut_invalid_index() {
    let mut map = IndexMap::new();
    map.push("key1", "value1");

    let result = map.get_index_mut(2);
    assert!(result.is_none());
}

#[test]
fn test_get_index_mut_empty_map() {
    let mut map: IndexMap<&str, &str> = IndexMap::new();

    let result = map.get_index_mut(0);
    assert!(result.is_none());
}

