// Answer 0

#[test]
fn test_key_occupied_entry() {
    struct OccupiedEntry {
        key: String,
    }

    enum Entry {
        Vacant(VacantEntry),
        Occupied(OccupiedEntry),
    }

    struct VacantEntry {
        key: String,
    }

    impl OccupiedEntry {
        fn key(&self) -> &String {
            &self.key
        }
    }

    impl VacantEntry {
        fn key(&self) -> &String {
            &self.key
        }
    }

    impl Entry {
        pub fn key(&self) -> &String {
            match self {
                Entry::Vacant(e) => e.key(),
                Entry::Occupied(e) => e.key(),
            }
        }
    }

    let occupied_entry = OccupiedEntry {
        key: String::from("serde"),
    };
    
    let entry = Entry::Occupied(occupied_entry);
    assert_eq!(entry.key(), &"serde".to_string());
}

#[test]
#[should_panic]
fn test_key_vacant_entry() {
    struct OccupiedEntry {
        key: String,
    }

    enum Entry {
        Vacant(VacantEntry),
        Occupied(OccupiedEntry),
    }

    struct VacantEntry {
        key: String,
    }

    impl OccupiedEntry {
        fn key(&self) -> &String {
            &self.key
        }
    }

    impl VacantEntry {
        fn key(&self) -> &String {
            &self.key
        }
    }

    impl Entry {
        pub fn key(&self) -> &String {
            match self {
                Entry::Vacant(e) => e.key(),
                Entry::Occupied(e) => e.key(),
            }
        }
    }

    let vacant_entry = VacantEntry {
        key: String::from("unused"),
    };
    
    let entry = Entry::Vacant(vacant_entry);
    // This will panic because we are testing the behavior of Entry in the Vacant state
    let _ = entry.key(); 
}

