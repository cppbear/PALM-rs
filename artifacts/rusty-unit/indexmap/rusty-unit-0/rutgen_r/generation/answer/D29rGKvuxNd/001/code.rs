// Answer 0

#[test]
fn test_first_mut_non_empty() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    assert_eq!(map.first_mut(), Some((&"key1", &mut 1)));
}

#[test]
fn test_first_mut_empty() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();

    assert_eq!(map.first_mut(), None);
}

#[test]
fn test_first_mut_single_entry() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("only_key", 42);

    assert_eq!(map.first_mut(), Some((&"only_key", &mut 42)));
}

#[test]
#[should_panic]
fn test_first_mut_panic() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    assert_eq!(map.first_mut(), None); // ongoing access should validate no panic
}

