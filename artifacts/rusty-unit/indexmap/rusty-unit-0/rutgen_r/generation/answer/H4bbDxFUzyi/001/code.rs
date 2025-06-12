// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct Map<K, V> {
    entries: Vec<Entry<K, V>>,
    current_index: usize,
}

impl<K, V> Map<K, V> {
    fn new() -> Self {
        Map {
            entries: Vec::new(),
            current_index: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.entries.push(Entry { key, value });
    }

    fn index(&self) -> usize {
        self.current_index
    }

    fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {
        let index = self.index();
        let entry = &mut self.entries[index];
        (&mut entry.key, &mut entry.value)
    }
}

#[test]
fn test_get_key_value_mut_valid() {
    let mut map = Map::new();
    map.insert("key1", "value1");
    
    map.current_index = 0; // Set index to valid range
    
    let (key, value) = map.get_key_value_mut();
    
    assert_eq!(*key, "key1");
    assert_eq!(*value, "value1");
}

#[should_panic]
#[test]
fn test_get_key_value_mut_index_out_of_bounds() {
    let mut map = Map::new();
    map.insert("key1", "value1");
    
    map.current_index = 1; // Set index to out of bounds
    
    let _ = map.get_key_value_mut(); // Expect this to panic
}

#[test]
fn test_get_key_value_mut_multiple_entries() {
    let mut map = Map::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    map.current_index = 1; // Set index to valid range
    
    let (key, value) = map.get_key_value_mut();
    
    assert_eq!(*key, "key2");
    assert_eq!(*value, "value2");
}

