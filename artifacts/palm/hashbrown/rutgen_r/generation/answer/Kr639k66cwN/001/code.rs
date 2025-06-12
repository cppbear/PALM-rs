// Answer 0

#[test]
fn test_get_mut_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    let x = map.get_mut(&1);
    assert!(x.is_some());
    if let Some(value) = x {
        *value = "b";
    }
    assert_eq!(map[&1], "b");
}

#[test]
fn test_get_mut_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    let x = map.get_mut(&2);
    assert!(x.is_none());
}

