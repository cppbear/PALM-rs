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

    fn index(&self) -> usize {
        self.entries.len().saturating_sub(1)
    }

    pub fn key(&self) -> &K {
        &self.entries[self.index()].key
    }
}

#[test]
fn test_key_with_one_entry() {
    let mut map: Map<i32> = Map::new();
    map.insert(42);
    let key = map.key();
    assert_eq!(*key, 42);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_key_with_no_entries() {
    let map: Map<i32> = Map::new();
    let _key = map.key(); // This should panic
}

#[test]
fn test_key_with_multiple_entries() {
    let mut map: Map<String> = Map::new();
    map.insert("first".to_string());
    map.insert("second".to_string());
    let key = map.key();
    assert_eq!(*key, "second".to_string());
}

#[test]
fn test_key_after_insertion() {
    let mut map: Map<f64> = Map::new();
    map.insert(3.14);
    let first_key = map.key();
    assert_eq!(*first_key, 3.14);

    map.insert(2.71);
    let second_key = map.key();
    assert_eq!(*second_key, 2.71);
}

