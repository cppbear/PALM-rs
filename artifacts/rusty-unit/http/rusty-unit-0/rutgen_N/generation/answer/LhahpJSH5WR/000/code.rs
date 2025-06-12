// Answer 0

#[test]
fn test_keys_empty_header_map() {
    struct HeaderMap {
        entries: Vec<(String, String)>,
    }

    struct Keys<'a, T> {
        inner: std::slice::Iter<'a, (String, String)>,
    }

    impl HeaderMap {
        pub fn new() -> Self {
            HeaderMap { entries: vec![] }
        }

        pub fn keys(&self) -> Keys<'_, String> {
            Keys {
                inner: self.entries.iter(),
            }
        }
    }

    let map = HeaderMap::new();
    let mut keys: Vec<_> = map.keys().inner.map(|(key, _)| key).collect();
    assert!(keys.is_empty());
}

#[test]
fn test_keys_single_entry_header_map() {
    struct HeaderMap {
        entries: Vec<(String, String)>,
    }

    struct Keys<'a, T> {
        inner: std::slice::Iter<'a, (String, String)>,
    }

    impl HeaderMap {
        pub fn new() -> Self {
            HeaderMap { entries: vec![] }
        }

        pub fn insert(&mut self, key: String, value: String) {
            self.entries.push((key, value));
        }

        pub fn keys(&self) -> Keys<'_, String> {
            Keys {
                inner: self.entries.iter(),
            }
        }
    }

    let mut map = HeaderMap::new();
    map.insert("Content-Length".to_string(), "123".to_string());

    let mut keys: Vec<_> = map.keys().inner.map(|(key, _)| key).collect();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], "Content-Length");
}

#[test]
fn test_keys_multiple_entries_header_map() {
    struct HeaderMap {
        entries: Vec<(String, String)>,
    }

    struct Keys<'a, T> {
        inner: std::slice::Iter<'a, (String, String)>,
    }

    impl HeaderMap {
        pub fn new() -> Self {
            HeaderMap { entries: vec![] }
        }

        pub fn insert(&mut self, key: String, value: String) {
            self.entries.push((key, value));
        }

        pub fn keys(&self) -> Keys<'_, String> {
            Keys {
                inner: self.entries.iter(),
            }
        }
    }

    let mut map = HeaderMap::new();
    map.insert("Host".to_string(), "hello".to_string());
    map.insert("Content-Length".to_string(), "123".to_string());
    map.insert("Host".to_string(), "goodbye".to_string());

    let keys: Vec<_> = map.keys().inner.map(|(key, _)| key).collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"Host"));
    assert!(keys.contains(&"Content-Length"));
}

