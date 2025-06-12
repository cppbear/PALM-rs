// Answer 0

#[test]
fn test_get_key_value_mut_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    let (k, v) = map.get_key_value_mut(&1).unwrap();
    assert_eq!(k, &1);
    assert_eq!(v, &mut "a");
    *v = "b";
    assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "b")));
}

#[test]
fn test_get_key_value_mut_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value_mut(&2), None);
}

#[test]
fn test_get_key_value_mut_boundary_conditions() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(0, "first");
    map.insert(usize::MAX, "last");

    // Test getting existing keys
    assert_eq!(map.get_key_value_mut(&0), Some((&0, &mut "first")));
    assert_eq!(map.get_key_value_mut(&usize::MAX), Some((&usize::MAX, &mut "last")));

    // Test getting non-existing keys
    assert_eq!(map.get_key_value_mut(&1), None);
    assert_eq!(map.get_key_value_mut(&(usize::MAX - 1)), None);
}

