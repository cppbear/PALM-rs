// Answer 0

#[test]
fn test_replace_entry_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // First replace to ensure entry is occupied
    let entry = match map.entry("poneyland") {
        Entry::Occupied(e) => {
            e.replace_entry_with(|k, v| {
                assert_eq!(k, &"poneyland");
                assert_eq!(v, 42);
                // Returning None to trigger Vacant entry
                None
            })
        }
        Entry::Vacant(_) => panic!(),
    };

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"poneyland");
        }
        Entry::Occupied(_) => panic!(),
    }

    // Check that the entry is indeed removed from the map
    assert!(!map.contains_key("poneyland"));
}

#[test]
fn test_replace_entry_with_no_update() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("cloudland", 100);

    // Test replacing with a function that returns None
    let entry = match map.entry("cloudland") {
        Entry::Occupied(e) => {
            e.replace_entry_with(|k, v| {
                assert_eq!(k, &"cloudland");
                assert_eq!(v, 100);
                // Inducing a vacant entry
                None
            })
        }
        Entry::Vacant(_) => panic!(),
    };

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"cloudland");
        }
        Entry::Occupied(_) => panic!(),
    }

    // Ensure the map is updated correctly
    assert!(!map.contains_key("cloudland"));
}

