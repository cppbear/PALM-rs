// Answer 0

#[test]
fn test_key_on_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    match map.entry("poneyland") {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => assert_eq!(entry.key(), &"poneyland"),
    }
}

#[test]
fn test_key_on_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let map: HashMap<&str, u32> = HashMap::new();

    match map.entry("unicornland") {
        Entry::Vacant(_) => {}, // Here we expect it to be vacant
        Entry::Occupied(_) => panic!(),
    }
}

