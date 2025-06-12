// Answer 0

#[test]
fn test_and_replace_entry_with_occupied() {
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
fn test_and_replace_entry_with_occupied_return_none() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let entry = map
        .entry("poneyland")
        .and_replace_entry_with(|k, v| {
            assert_eq!(k, &"poneyland");
            assert_eq!(v, 42);
            None
        });

    match entry {
        Entry::Vacant(e) => {
            assert_eq!(e.key(), &"poneyland");
        }
        Entry::Occupied(_) => panic!(),
    }

    assert!(!map.contains_key("poneyland"));
}

#[test]
#[should_panic]
fn test_and_replace_entry_with_panic() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    let _ = map
        .entry("poneyland")
        .and_replace_entry_with(|_k, _v| panic!());
}

