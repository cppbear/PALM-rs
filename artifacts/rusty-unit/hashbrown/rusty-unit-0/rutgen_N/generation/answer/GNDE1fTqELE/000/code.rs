// Answer 0

#[test]
fn test_and_replace_entry_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .entry("poneyland")
        .and_replace_entry_with(|_k, _v| panic!());

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"poneyland");
        }
        Entry::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let entry = map
        .entry("poneyland")
        .and_replace_entry_with(|k, v| {
            assert_eq!(k, &"poneyland");
            assert_eq!(v, 42);
            Some(v + 1)
        });

    match entry {
        Entry::Occupied(e) => {
            assert_eq!(e.key(), &"poneyland");
            assert_eq!(e.get(), &43);
        }
        Entry::Vacant(_) => panic!(),
    }

    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_replace_entry_with_remove_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let entry = map
        .entry("poneyland")
        .and_replace_entry_with(|_k, _v| None);

    match entry {
        Entry::Vacant(e) => assert_eq!(e.key(), &"poneyland"),
        Entry::Occupied(_) => panic!(),
    }

    assert!(!map.contains_key("poneyland"));
}

