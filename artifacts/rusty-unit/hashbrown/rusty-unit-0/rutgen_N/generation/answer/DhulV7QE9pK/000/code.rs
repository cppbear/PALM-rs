// Answer 0

#[test]
fn test_insert_new_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    assert_eq!(map.insert(1, "first"), None);
    assert_eq!(map.is_empty(), false);
    assert_eq!(map[&1], "first");
}

#[test]
fn test_insert_existing_key_update_value() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "first");
    assert_eq!(map.insert(1, "updated"), Some("first"));
    assert_eq!(map[&1], "updated");
}

#[test]
fn test_insert_overwrite_value() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(2, "original");
    map.insert(2, "new");
    assert_eq!(map.insert(2, "overwrite"), Some("new"));
    assert_eq!(map[&2], "overwrite");
}

#[test]
fn test_insert_different_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    assert_eq!(map.insert(1, "one"), None);
    assert_eq!(map.insert(2, "two"), None);
    assert_eq!(map[&1], "one");
    assert_eq!(map[&2], "two");
}

#[test]
fn test_insert_multiple_updates() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(3, "initial");
    assert_eq!(map.insert(3, "first_update"), Some("initial"));
    assert_eq!(map.insert(3, "second_update"), Some("first_update"));
    assert_eq!(map[&3], "second_update");
}

