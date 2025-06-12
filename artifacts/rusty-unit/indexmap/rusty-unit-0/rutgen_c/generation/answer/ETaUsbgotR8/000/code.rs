// Answer 0

#[test]
fn test_get_full_mut2_existing_key() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[derive(Hash, PartialEq, Eq)]
    struct Key(u32);
    struct Value(String);

    let mut map = IndexMap::<Key, Value, DefaultHasher>::new();
    map.insert(Key(1), Value("value1".into()));

    let result = map.get_full_mut2(&Key(1));
    assert!(result.is_some());
    if let Some((index, key_ref, value_ref)) = result {
        assert_eq!(index, 0);
        assert_eq!(key_ref.0, 1);
        assert_eq!(value_ref.0, "value1");
    }
}

#[test]
fn test_get_full_mut2_non_existent_key() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[derive(Hash, PartialEq, Eq)]
    struct Key(u32);
    struct Value(String);

    let mut map = IndexMap::<Key, Value, DefaultHasher>::new();
    map.insert(Key(1), Value("value1".into()));

    let result = map.get_full_mut2(&Key(2));
    assert!(result.is_none());
}

#[test]
fn test_get_full_mut2_empty_map() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[derive(Hash, PartialEq, Eq)]
    struct Key(u32);
    struct Value(String);

    let mut map = IndexMap::<Key, Value, DefaultHasher>::new();

    let result = map.get_full_mut2(&Key(1));
    assert!(result.is_none());
}

