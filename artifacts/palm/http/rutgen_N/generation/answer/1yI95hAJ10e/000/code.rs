// Answer 0

#[test]
fn test_is_empty_with_empty_header_map() {
    struct HeaderMap {
        entries: Vec<String>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { entries: Vec::new() }
        }

        fn is_empty(&self) -> bool {
            self.entries.len() == 0
        }

        fn insert(&mut self, entry: String) {
            self.entries.push(entry);
        }
    }

    let map = HeaderMap::new();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_header_map() {
    struct HeaderMap {
        entries: Vec<String>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { entries: Vec::new() }
        }

        fn is_empty(&self) -> bool {
            self.entries.len() == 0
        }

        fn insert(&mut self, entry: String) {
            self.entries.push(entry);
        }
    }

    let mut map = HeaderMap::new();
    map.insert(String::from("hello.world"));
    assert!(!map.is_empty());
}

