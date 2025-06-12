// Answer 0

#[test]
fn test_from_key_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    use super::{IndexMap, RawEntryBuilder};
    
    let mut map: IndexMap<i32, &str, _> = IndexMap::with_hasher(DefaultHasher::new());
    map.insert(1, "one");
    map.insert(2, "two");

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key(&1);
    
    assert_eq!(result, Some((&1, &"one")));
}

#[test]
fn test_from_key_non_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    use super::{IndexMap, RawEntryBuilder};
    
    let mut map: IndexMap<i32, &str, _> = IndexMap::with_hasher(DefaultHasher::new());
    map.insert(1, "one");

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key(&2);
    
    assert_eq!(result, None);
}

#[test]
fn test_from_key_with_different_type() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    use super::{IndexMap, RawEntryBuilder};
    
    let mut map: IndexMap<String, i32, _> = IndexMap::with_hasher(DefaultHasher::new());
    map.insert("first".to_string(), 10);
    
    let builder = RawEntryBuilder { map: &map };
    let key: &str = "first";
    let result = builder.from_key(&key);
    
    assert_eq!(result, Some((&"first".to_string(), &10)));
}

#[test]
fn test_from_key_empty_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    use super::{IndexMap, RawEntryBuilder};
    
    let map: IndexMap<i32, &str, _> = IndexMap::with_hasher(DefaultHasher::new());

    let builder = RawEntryBuilder { map: &map };
    let result = builder.from_key(&1);
    
    assert_eq!(result, None);
}

