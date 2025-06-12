// Answer 0


#[derive(Debug)]
struct DummyBucket<K, V> {
    key: K,
    value: V,
}

struct DummyMap<K, V> {
    entries: Vec<DummyBucket<K, V>>,
}

impl<K, V> DummyMap<K, V> {
    pub fn new() -> Self {
        DummyMap {
            entries: Vec::new(),
        }
    }
    
    pub fn push(&mut self, key: K, value: V) {
        self.entries.push(DummyBucket { key, value });
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn as_entries(&self) -> &Vec<DummyBucket<K, V>> {
        &self.entries
    }
    
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.as_entries().get(index).map(|bucket| (&bucket.key, &bucket.value))
    }
}

#[test]
fn test_get_index_valid_first() {
    let mut map = DummyMap::new();
    map.push("key1", "value1");

    let result = map.get_index(0);
    assert_eq!(result, Some((&"key1", &"value1")));
}

#[test]
fn test_get_index_valid_second() {
    let mut map = DummyMap::new();
    map.push("key1", "value1");
    map.push("key2", "value2");

    let result = map.get_index(1);
    assert_eq!(result, Some((&"key2", &"value2")));
}

#[test]
fn test_get_index_out_of_bounds_low() {
    let map: DummyMap<i32, i32> = DummyMap::new();
    
    let result = map.get_index(0);
    assert_eq!(result, None);
}

#[test]
fn test_get_index_out_of_bounds_high() {
    let mut map = DummyMap::new();
    map.push("key1", "value1");

    let result = map.get_index(1);
    assert_eq!(result, None);
}

#[test]
fn test_get_index_multiple_entries() {
    let mut map = DummyMap::new();
    map.push(1, 100);
    map.push(2, 200);
    map.push(3, 300);

    let result = map.get_index(2);
    assert_eq!(result, Some((&3, &300)));
}


