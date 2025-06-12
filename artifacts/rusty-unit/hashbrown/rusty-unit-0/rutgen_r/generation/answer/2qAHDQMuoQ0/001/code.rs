// Answer 0

#[test]
fn test_into_mut_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    let value: &mut u32;
    match map.entry("poneyland") {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
    
    *value += 10;

    assert_eq!(map["poneyland"], 22);
}

#[test]
#[should_panic(expected = "Expected an occupied entry")]
fn test_into_mut_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let value: &mut u32;
    match map.entry("nonexistent") {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

