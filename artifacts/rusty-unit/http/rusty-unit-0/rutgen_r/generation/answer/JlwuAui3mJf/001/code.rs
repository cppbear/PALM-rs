// Answer 0

#[test]
fn test_key_vacant_entry() {
    use http::{HeaderMap, HeaderName};

    struct VacantEntry {
        key: HeaderName,
    }

    impl VacantEntry {
        fn key(&self) -> &HeaderName {
            &self.key
        }
    }

    enum Entry {
        Vacant(VacantEntry),
        Occupied,
    }

    let vacant_entry = VacantEntry {
        key: HeaderName::from_static("x-hello"),
    };

    let entry = Entry::Vacant(vacant_entry);
    
    let key = match entry {
        Entry::Vacant(ref e) => e.key(),
        Entry::Occupied => panic!("Expected a Vacant entry"),
    };

    assert_eq!(key.as_str(), "x-hello");
} 

#[test]
fn test_key_occupied_entry() {
    use http::{HeaderMap, HeaderName};

    struct OccupiedEntry {
        key: HeaderName,
    }

    impl OccupiedEntry {
        fn key(&self) -> &HeaderName {
            &self.key
        }
    }

    enum Entry {
        Vacant,
        Occupied(OccupiedEntry),
    }

    let occupied_entry = OccupiedEntry {
        key: HeaderName::from_static("x-world"),
    };

    let entry = Entry::Occupied(occupied_entry);
    
    let key = match entry {
        Entry::Vacant => panic!("Expected an Occupied entry"),
        Entry::Occupied(ref e) => e.key(),
    };

    assert_eq!(key.as_str(), "x-world");
}

