// Answer 0

#[derive(Debug)]
struct Entry<K> {
    key: K,
}

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

    fn into_key(&mut self, index: usize) -> &mut K {
        let index = index;
        &mut self.entries[index].key
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

#[test]
fn test_into_key_valid() {
    let mut map: Map<i32> = Map::new();
    map.insert(42);
    map.insert(73);
    
    let key: &mut i32 = map.into_key(0);
    *key += 1; // Change value to ensure we have mutable reference
    assert_eq!(map.entries[0].key, 43);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_into_key_panic_out_of_bounds() {
    let mut map: Map<i32> = Map::new();
    map.insert(42);
    
    let _key: &mut i32 = map.into_key(1); // This should panic
}

#[test]
fn test_into_key_multiple_entries() {
    let mut map: Map<String> = Map::new();
    map.insert("first".to_string());
    map.insert("second".to_string());

    let key: &mut String = map.into_key(1);
    key.push_str(" updated");
    assert_eq!(map.entries[1].key, "second updated");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_into_key_multiple_entries_panic() {
    let mut map: Map<i32> = Map::new();
    map.insert(1);
    map.insert(2);
    
    let _key: &mut i32 = map.into_key(2); // Panic due to out of bounds index
}

