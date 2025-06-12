// Answer 0

#[derive(Debug)]
struct MyMap<K, V> {
    data: std::collections::HashMap<K, V>,
}

impl<K, V> MyMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        MyMap {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + std::hash::Hash + std::cmp::PartialEq<K>,
    {
        self.data.contains_key(key)
    }
}

#[test]
fn test_contains_key_existing() {
    let mut map = MyMap::new();
    map.insert("key1", 1);
    assert!(map.contains_key(&"key1"));
}

#[test]
fn test_contains_key_non_existing() {
    let mut map = MyMap::new();
    map.insert("key1", 1);
    assert!(!map.contains_key(&"key2"));
}

#[test]
fn test_contains_key_empty() {
    let map: MyMap<&str, i32> = MyMap::new();
    assert!(!map.contains_key(&"key1"));
}

