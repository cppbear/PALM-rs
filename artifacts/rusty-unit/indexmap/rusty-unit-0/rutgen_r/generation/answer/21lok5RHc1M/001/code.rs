// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

impl<K, V> Bucket<K, V> {
    pub fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
}

#[derive(Debug)]
struct IndexMap<K, V> {
    entries: Vec<Bucket<K, V>>,
}

impl<K, V> IndexMap<K, V> {
    pub fn new() -> Self {
        IndexMap { entries: Vec::new() }
    }

    pub fn last(&self) -> Option<(&K, &V)> {
        self.entries.last().map(Bucket::refs)
    }

    pub fn push(&mut self, key: K, value: V) {
        self.entries.push(Bucket { key, value });
    }
}

#[test]
fn test_last_on_empty_map() {
    let map: IndexMap<i32, String> = IndexMap::new();
    assert_eq!(map.last(), None);
}

#[test]
fn test_last_on_single_element_map() {
    let mut map = IndexMap::new();
    map.push(1, "one".to_string());
    assert_eq!(map.last(), Some((&1, &"one".to_string())));
}

#[test]
fn test_last_on_multiple_elements_map() {
    let mut map = IndexMap::new();
    map.push(1, "one".to_string());
    map.push(2, "two".to_string());
    map.push(3, "three".to_string());
    assert_eq!(map.last(), Some((&3, &"three".to_string())));
}

#[test]
fn test_last_with_different_types() {
    let mut map = IndexMap::new();
    map.push("key1", 100);
    map.push("key2", 200);
    assert_eq!(map.last(), Some((&"key2", &200)));
}

