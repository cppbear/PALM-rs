// Answer 0

#[test]
fn test_insert_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        assert_eq!(o.insert(15), 12);
    }

    assert_eq!(map["poneyland"], 15);
}

#[test]
fn test_insert_non_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(v) = map.entry("unicornland") {
        v.insert(20);
    }

    assert_eq!(map["unicornland"], 20);
}

#[test]
fn test_insert_entry_edge_case() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 0);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        assert_eq!(o.insert(0), 0);
    }

    assert_eq!(map["poneyland"], 0);
}

