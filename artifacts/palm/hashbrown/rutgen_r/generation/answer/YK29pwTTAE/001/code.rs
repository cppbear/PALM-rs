// Answer 0

#[test]
fn test_contains_key_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    assert_eq!(map.contains_key(&1), true);
}

#[test]
fn test_contains_key_non_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");

    assert_eq!(map.contains_key(&2), false);
}

#[test]
fn test_contains_key_with_borrowed_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("key", 42);

    let borrowed_key: &str = "key";
    assert_eq!(map.contains_key(borrowed_key), true);
}

#[test]
fn test_contains_key_with_unsized_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, i32> = HashMap::new();
    let key = String::from("key");
    map.insert(key.clone(), 10);

    let borrowed_key: &str = &key;
    assert_eq!(map.contains_key(borrowed_key), true);
}

#[test]
#[should_panic]
fn test_contains_key_panics_on_non_hashable_key() {
    use hashbrown::HashMap;

    struct NonHashable;
    
    let mut map: HashMap<NonHashable, i32> = HashMap::new();
    
    // The following statement should panic if you try to use a non-hashable key type.
    assert_eq!(map.contains_key(&NonHashable), false);
}

