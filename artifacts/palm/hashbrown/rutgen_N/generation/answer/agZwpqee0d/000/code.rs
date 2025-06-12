// Answer 0

#[test]
fn test_from_key_existing_entry() {
    use hashbrown::HashMap;

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "a";
    assert_eq!(map.raw_entry().from_key(&key), Some((&"a", &100)));
}

#[test]
fn test_from_key_non_existing_entry() {
    use hashbrown::HashMap;

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "c";
    assert_eq!(map.raw_entry().from_key(&key), None);
}

#[test]
fn test_from_key_boundary_case_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<&str, u32> = HashMap::new();
    let key = "a";
    assert_eq!(map.raw_entry().from_key(&key), None);
}

