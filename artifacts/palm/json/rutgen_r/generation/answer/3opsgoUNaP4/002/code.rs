// Answer 0

#[test]
fn test_entry_key_vacant() {
    struct VacantEntry {
        key_value: String,
    }
    
    impl VacantEntry {
        fn key(&self) -> &String {
            &self.key_value
        }
    }
    
    enum Entry {
        Vacant(VacantEntry),
        Occupied(String),
    }
    
    let vacant_entry = VacantEntry {
        key_value: String::from("serde"),
    };
    
    let entry = Entry::Vacant(vacant_entry);
    
    let key = match entry {
        Entry::Vacant(e) => e.key(),
        Entry::Occupied(_) => unreachable!(),
    };
    
    assert_eq!(key, &"serde".to_string());
}

#[test]
fn test_entry_key_occupied() {
    struct OccupiedEntry {
        key_value: String,
    }
    
    impl OccupiedEntry {
        fn key(&self) -> &String {
            &self.key_value
        }
    }
    
    enum Entry {
        Vacant(VacantEntry),
        Occupied(OccupiedEntry),
    }
    
    let occupied_entry = OccupiedEntry {
        key_value: String::from("serde"),
    };
    
    let entry = Entry::Occupied(occupied_entry);
    
    let key = match entry {
        Entry::Vacant(_) => unreachable!(),
        Entry::Occupied(e) => e.key(),
    };
    
    assert_eq!(key, &"serde".to_string());
}

