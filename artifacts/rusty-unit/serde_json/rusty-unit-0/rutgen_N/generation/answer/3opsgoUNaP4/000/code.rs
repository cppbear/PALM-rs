// Answer 0

#[test]
fn test_entry_key_occupied() {
    struct OccupiedEntry {
        key: String,
    }

    impl OccupiedEntry {
        fn key(&self) -> &String {
            &self.key
        }
    }

    enum Entry {
        Vacant,
        Occupied(OccupiedEntry),
    }

    let entry = Entry::Occupied(OccupiedEntry { key: "serde".to_string() });
    if let Entry::Occupied(e) = entry {
        assert_eq!(e.key(), &"serde".to_string());
    }
}

#[test]
fn test_entry_key_vacant() {
    struct VacantEntry {
        key: String,
    }

    impl VacantEntry {
        fn key(&self) -> &String {
            &self.key
        }
    }

    enum Entry {
        Vacant(VacantEntry),
        Occupied,
    }

    let entry = Entry::Vacant(VacantEntry { key: "serde".to_string() });
    if let Entry::Vacant(e) = entry {
        assert_eq!(e.key(), &"serde".to_string());
    }
}

