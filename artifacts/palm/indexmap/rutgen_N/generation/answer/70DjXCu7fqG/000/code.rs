// Answer 0

#[test]
fn test_get_index_of_empty_map() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let map: IndexMap<u32, &str> = IndexMap::new();
    let key = 1;
    assert_eq!(map.get_index_of(&key), None);
}

#[test]
fn test_get_index_of_single_entry() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let mut map = IndexMap::new();
    map.insert(1, "one");
    let key = 1;
    assert_eq!(map.get_index_of(&key), Some(0));

    let missing_key = 2;
    assert_eq!(map.get_index_of(&missing_key), None);
}

#[test]
fn test_get_index_of_multiple_entries() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let mut map = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let key = 2;
    assert_eq!(map.get_index_of(&key), Some(1));

    let missing_key = 4;
    assert_eq!(map.get_index_of(&missing_key), None);
}

