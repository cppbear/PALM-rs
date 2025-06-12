// Answer 0

#[test]
fn test_get_key_value_exists() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");

    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(map.get_key_value(&2), Some((&2, &"b")));
}

#[test]
fn test_get_key_value_not_exists() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    assert_eq!(map.get_key_value(&2), None);
}

#[test]
fn test_get_key_value_with_different_ref() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(10, "ten");

    let key_ref: &i32 = &10;
    assert_eq!(map.get_key_value(key_ref), Some((&10, &"ten")));
}

#[test]
fn test_get_key_value_with_negative_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(-1, "negative one");

    assert_eq!(map.get_key_value(&-1), Some((&-1, &"negative one")));
}

