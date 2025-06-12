// Answer 0

#[test]
fn test_from_key_with_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, &str, DefaultHasher> = IndexMap::new();
    map.insert(1, "value1");
    map.insert(2, "value2");

    let entry = map.raw_entry_mut().from_key(&1);
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().key(), &1);
}

#[test]
fn test_from_key_with_non_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, &str, DefaultHasher> = IndexMap::new();
    map.insert(1, "value1");
    map.insert(2, "value2");

    let entry = map.raw_entry_mut().from_key(&3);
    assert!(entry.is_none());
}

