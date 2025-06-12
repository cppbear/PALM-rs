// Answer 0

#[test]
fn test_key_occupied_entry() {
    struct OccupiedEntry {
        key_value: String,
    }
    
    impl OccupiedEntry {
        fn key(&self) -> &String {
            &self.key_value
        }
    }
    
    enum Entry {
        Vacant(OccupiedEntry),
        Occupied(OccupiedEntry),
    }
    
    let occupied_entry = OccupiedEntry { key_value: String::from("serde") };
    let entry = Entry::Occupied(occupied_entry);
    
    match entry {
        Entry::Occupied(e) => assert_eq!(e.key(), &"serde"),
        _ => panic!("Expected Entry::Occupied"),
    }
}

#[test]
#[should_panic] // to test the panic condition when using Vacant entry
fn test_key_vacant_entry() {
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
        Occupied(VacantEntry),
    }
    
    let vacant_entry = VacantEntry { key_value: String::from("serde") };
    let entry = Entry::Vacant(vacant_entry);
    
    match entry {
        Entry::Occupied(_) => unreachable!(),
        Entry::Vacant(e) => {
            // This is just to trigger a panic by trying to mimic the call to Occupied entry key()
            let _ = e.key(); // This will not panic but will not match the expected case above
        }
    }
}

