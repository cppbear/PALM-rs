// Answer 0

fn test_key_mut_occupied() {
    struct OccupiedEntry {
        key: String,
    }

    impl OccupiedEntry {
        fn key_mut(&mut self) -> &mut String {
            &mut self.key
        }
    }

    enum Entry {
        Occupied(OccupiedEntry),
        Vacant,
    }

    let mut entry = Entry::Occupied(OccupiedEntry {
        key: String::from("test_key"),
    });

    if let Entry::Occupied(ref mut e) = entry {
        let key_ref = e.key_mut();
        assert_eq!(key_ref, &mut String::from("test_key"));
        key_ref.push_str("_updated");
    }

    if let Entry::Occupied(e) = entry {
        assert_eq!(e.key, "test_key_updated");
    }
}

fn test_key_mut_vacant() {
    struct VacantEntry {
        key: String,
    }

    impl VacantEntry {
        fn key_mut(&mut self) -> &mut String {
            &mut self.key
        }
    }

    enum Entry {
        Occupied(VacantEntry), // Changing Occupied to Vacant for this test
        Vacant(VacantEntry),
    }

    let mut entry = Entry::Vacant(VacantEntry {
        key: String::from("vacant_key"),
    });

    if let Entry::Vacant(ref mut e) = entry {
        let key_ref = e.key_mut();
        assert_eq!(key_ref, &mut String::from("vacant_key"));
        key_ref.push_str("_updated");
    }

    if let Entry::Vacant(e) = entry {
        assert_eq!(e.key, "vacant_key_updated");
    }
}

