// Answer 0

#[test]
fn test_into_key_vacant_entry() {
    use http::header::{HeaderMap, Entry, HeaderName};

    struct VacantEntry {
        key: HeaderName,
    }

    impl VacantEntry {
        fn new(key: HeaderName) -> Self {
            VacantEntry { key }
        }
        
        fn into_key(self) -> HeaderName {
            self.key
        }
    }

    let mut map = HeaderMap::new();

    if let Entry::Vacant(v) = map.entry("x-hello") {
        let vacant_entry = VacantEntry::new(v.into_key());
        assert_eq!(vacant_entry.into_key().as_str(), "x-hello");
    }
}

#[test]
fn test_into_key_with_different_key() {
    use http::header::{HeaderMap, Entry, HeaderName};

    struct VacantEntry {
        key: HeaderName,
    }

    impl VacantEntry {
        fn new(key: HeaderName) -> Self {
            VacantEntry { key }
        }
        
        fn into_key(self) -> HeaderName {
            self.key
        }
    }

    let mut map = HeaderMap::new();

    if let Entry::Vacant(v) = map.entry("x-world") {
        let vacant_entry = VacantEntry::new(v.into_key());
        assert_eq!(vacant_entry.into_key().as_str(), "x-world");
    }
}

