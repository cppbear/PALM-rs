// Answer 0

#[test]
fn test_entry_get_occupied() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    match map.entry("poneyland") {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            assert_eq!(entry.get(), &12);
        }
    }
}

#[test]
fn test_entry_get_vacant() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.entry("nonexistent") {
        Entry::Vacant(entry) => {
            entry.insert(15);
            assert_eq!(map.get("nonexistent"), Some(&15));
        },
        Entry::Occupied(_) => panic!(),
    }
}

