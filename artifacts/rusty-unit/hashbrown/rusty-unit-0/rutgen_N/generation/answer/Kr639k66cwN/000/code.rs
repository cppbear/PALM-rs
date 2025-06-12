// Answer 0

#[test]
fn test_get_mut_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    assert_eq!(map[&1], "b");
}

#[test]
fn test_get_mut_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_mut(&2), None);
}

#[test]
fn test_get_mut_with_different_borrowed_form() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(2, "c");
    if let Some(x) = map.get_mut(&2) {
        *x = "d";
    }
    assert_eq!(map[&2], "d");
}

#[test]
fn test_get_mut_on_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    assert_eq!(map.get_mut(&1), None);
}

