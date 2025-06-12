// Answer 0

#[test]
fn test_insert_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        assert_eq!(o.insert(15), 12);
    }
    assert_eq!(map["poneyland"], 15);
}

#[test]
fn test_insert_multiple_entries() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);
    map.insert("wonderland", 20);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        assert_eq!(o.insert(25), 12);
    }
    assert_eq!(map["poneyland"], 25);
    assert_eq!(map["wonderland"], 20);
}

#[test]
fn test_insert_empty_map() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(v) = map.entry("poneyland") {
        v.insert(30);
    }
    assert_eq!(map["poneyland"], 30);
}

#[test]
fn test_insert_with_overwriting() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("magicland", 100);

    if let Entry::Occupied(mut o) = map.entry("magicland") {
        assert_eq!(o.insert(200), 100);
    }
    assert_eq!(map["magicland"], 200);
}

#[test]
#[should_panic]
fn test_insert_on_non_existent_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // This will panic since we're not guaranteeing that the entry exists.
    let _ = map.entry("nonexistentland").or_insert_with(|| 50);
    if let Entry::Occupied(mut o) = map.entry("nonexistentland") {
        o.insert(60);
    }
}

