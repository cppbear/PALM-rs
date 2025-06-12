// Answer 0

#[derive(Debug)]
struct HashValue(u64);

#[derive(Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

#[derive(Debug)]
struct Map<K, V> {
    entries: Vec<Bucket<K, V>>,
}

impl<K, V> Map<K, V> {
    fn new() -> Self {
        Map {
            entries: Vec::with_capacity(10),
        }
    }

    fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
        if self.entries.len() == self.entries.capacity() {
            self.entries.reserve(1);
        }
        self.entries.push(Bucket { hash, key, value });
    }
}

#[test]
fn test_push_entry_adds_entry() {
    let mut map: Map<&str, i32> = Map::new();
    let hash = HashValue(1);
    map.push_entry(hash, "key1", 100);

    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, "key1");
    assert_eq!(map.entries[0].value, 100);
}

#[test]
fn test_push_entry_reserves_capacity() {
    let mut map: Map<&str, i32> = Map::new();
    for i in 0..10 {
        map.push_entry(HashValue(i), &format!("key{}", i), i as i32);
    }
    let initial_capacity = map.entries.capacity();
    map.push_entry(HashValue(10), "key10", 100);
    
    assert!(map.entries.len() > initial_capacity);
}

