// Answer 0

#[test]
fn test_or_insert_nonexistent_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.entry_ref("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.entry_ref("poneyland").or_insert(3);
    *map.entry_ref("poneyland").or_insert(10) *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_boundary_condition() {
    use hashbrown::HashMap;

    let mut map: HashMap<u32, String> = HashMap::new();
    assert_eq!(map.entry_ref(1).or_insert("default".to_string()), "default");
    assert_eq!(map[&1], "default".to_string());
}

