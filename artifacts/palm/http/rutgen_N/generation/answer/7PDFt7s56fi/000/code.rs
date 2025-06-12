// Answer 0

#[test]
fn test_key() {
    struct HeaderName {
        name: String,
    }
    
    struct Entry {
        key: HeaderName,
    }

    struct HeaderMap {
        entries: Vec<Entry>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { entries: Vec::new() }
        }

        fn entry(&mut self, name: &str) -> &Entry {
            let header_name = HeaderName { name: name.to_string() };
            self.entries.push(Entry { key: header_name });
            self.entries.last().unwrap()
        }
    }

    impl Entry {
        fn key(&self) -> &HeaderName {
            &self.key
        }
    }

    let mut map = HeaderMap::new();
    let entry = map.entry("x-hello");
    
    assert_eq!(entry.key().name.as_str(), "x-hello");
}

