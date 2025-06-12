// Answer 0

#[test]
fn test_iter_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let table: HashTable<&str, &str, DefaultHashBuilder> = HashTable::new();
    let mut iter = table.iter();
    
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_element() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_fn(&"a"), "b", hasher_fn);

    let mut iter = table.iter();
    assert_eq!(iter.next(), Some(&"b"));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_fn(&"a"), "b", hasher_fn);
    table.insert_unique(hasher_fn(&"b"), "c", hasher_fn);
    
    let mut collected: Vec<_> = table.iter().collect();
    collected.sort(); // sorting to check expected values regardless of order
    assert_eq!(collected, vec![&"b", &"c"]);
}

#[test]
fn test_iter_no_duplicates() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_fn(&"key1"), "value1", hasher_fn);
    table.insert_unique(hasher_fn(&"key2"), "value2", hasher_fn);
    table.insert_unique(hasher_fn(&"key3"), "value3", hasher_fn);

    let mut iter = table.iter();
    let values: Vec<_> = iter.collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&&"value1"));
    assert!(values.contains(&&"value2"));
    assert!(values.contains(&&"value3"));
}

