// Answer 0

#[derive(Debug)]
struct HeaderMap<T> {
    entries: Vec<HeaderEntry<T>>,
}

#[derive(Debug)]
struct HeaderEntry<T> {
    value: T,
}

trait AsHeaderName {
    fn find<'a, T>(&self, map: &'a HeaderMap<T>) -> Option<(usize, usize)>;
}

impl AsHeaderName for String {
    fn find<'a, T>(&self, map: &'a HeaderMap<T>) -> Option<(usize, usize)> {
        for (index, entry) in map.entries.iter().enumerate() {
            if self == &entry.value {
                return Some((index, index));
            }
        }
        None
    }
}

impl<T> HeaderMap<T> {
    fn new() -> Self {
        HeaderMap {
            entries: Vec::new(),
        }
    }

    fn insert(&mut self, value: T) {
        self.entries.push(HeaderEntry { value });
    }

    fn get2<K>(&self, key: &K) -> Option<&T>
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
fn test_get2_found() {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type".to_string());
    
    let key = "Content-Type".to_string();
    let result = headers.get2(&key);
    assert_eq!(result, Some(&"Content-Type".to_string()));
}

#[test]
fn test_get2_not_found() {
    let headers = HeaderMap::new();
    
    let key = "Content-Type".to_string();
    let result = headers.get2(&key);
    assert_eq!(result, None);
}

