// Answer 0

#[derive(Debug)]
struct HeaderMap {
    entries: Vec<HeaderEntry>,
}

#[derive(Debug)]
struct HeaderEntry {
    value: String,
}

trait AsHeaderName {
    fn find(&self, map: &HeaderMap) -> Option<(usize, usize)>;
}

impl AsHeaderName for String {
    fn find(&self, map: &HeaderMap) -> Option<(usize, usize)> {
        for (index, entry) in map.entries.iter().enumerate() {
            if self == &entry.value {
                return Some((index, index));
            }
        }
        None
    }
}

impl HeaderMap {
    fn new() -> Self {
        HeaderMap { entries: Vec::new() }
    }
    
    fn insert(&mut self, value: String) {
        self.entries.push(HeaderEntry { value });
    }

    fn get2<K>(&self, key: &K) -> Option<&String>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((_, found)) => {
                let entry = &self.entries[found];
                Some(&entry.value)
            }
            None => None,
        }
    }
}

#[test]
fn test_get2_existing_key() {
    let mut map = HeaderMap::new();
    map.insert("Content-Type".to_string());

    let key = "Content-Type".to_string();
    let result = map.get2(&key);
    assert_eq!(result, Some(&"Content-Type".to_string()));
}

#[test]
fn test_get2_non_existing_key() {
    let mut map = HeaderMap::new();
    map.insert("Authorization".to_string());

    let key = "Content-Length".to_string();
    let result = map.get2(&key);
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_get2_panic_on_index_out_of_bounds() {
    let map = HeaderMap { entries: Vec::new() }; // Empty map
    let key = "Non-Existent-Key".to_string();
    let _ = map.get2(&key); // This should panic when trying to access entries[found].
}

