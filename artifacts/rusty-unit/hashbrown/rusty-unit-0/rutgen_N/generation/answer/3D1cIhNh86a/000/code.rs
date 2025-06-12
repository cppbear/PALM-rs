// Answer 0

#[test]
fn test_get_mut_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        *o.get_mut() += 10;
        assert_eq!(*o.get(), 22);

        // We can use the same Entry multiple times.
        *o.get_mut() += 2;
    }

    assert_eq!(map["poneyland"], 24);
}

#[test]
fn test_get_mut_non_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(v) = map.entry("poneyland") {
        v.insert(12);
    }

    assert_eq!(map["poneyland"], 12);
}

#[test]
#[should_panic]
fn test_get_mut_on_non_existent_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Occupied(o) = map.entry("poneyland") {
        let _ = o.get_mut(); // Should panic since the entry does not exist yet
    }
}

