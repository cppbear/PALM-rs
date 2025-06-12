// Answer 0

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
        Occupied(Vec<String>), // Dummy type to satisfy enum variant
    }

    let vacant_entry = VacantEntry {
        key: "serde".to_string(),
    };

    let entry = Entry::Vacant(vacant_entry);
    
    if let Entry::Vacant(ref e) = entry {
        assert_eq!(e.key(), &"serde".to_string());
    }
}

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
        Vacant(VacantEntry), // Dummy type to satisfy enum variant
        Occupied(OccupiedEntry),
    }

    let occupied_entry = OccupiedEntry {
        key: "serde".to_string(),
    };

    let entry = Entry::Occupied(occupied_entry);
    
    if let Entry::Occupied(ref e) = entry {
        assert_eq!(e.key(), &"serde".to_string());
    }
}

