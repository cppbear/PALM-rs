// Answer 0

#[test]
fn test_and_replace_entry_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("poneyland").and_replace_entry_with(|_k, _v| panic!());

    match entry {
        Entry::Vacant(e) => {
            // e.key() would be used in a real test case
        }
        Entry::Occupied(_) => panic!(),
    }

    // Adding "poneyland" to the map after 
    map.insert("poneyland", 42);
}

#[test]
fn test_and_replace_entry_with_vacant_entry_after_insertion() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);
    let entry = map.entry("poneyland").and_replace_entry_with(|k, v| {
        // Here we ensure that the key and value are as expected
        assert_eq!(k, &"poneyland");
        assert_eq!(v, 42);
        Some(v + 1)
    });

    match entry {
        Entry::Occupied(e) => {
            // e.key() and e.get() would be used in a real test case
        }
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_entry_removal() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("poneyland").and_replace_entry_with(|_k, _v| None);

    match entry {
        Entry::Vacant(e) => {
            // e.key() would be used in a real test case
        }
        Entry::Occupied(_) => panic!(),
    }

    assert!(!map.contains_key("poneyland"));
}

