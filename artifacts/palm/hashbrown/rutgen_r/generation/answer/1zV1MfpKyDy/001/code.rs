// Answer 0

#[test]
fn test_iter_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let table: HashTable<&str, &str, DefaultHashBuilder> = HashTable::new();
    let iter = table.iter();
    let collected: Vec<_> = iter.collect();
    assert!(collected.is_empty());
}

#[test]
fn test_iter_single_element() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_func = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_func(&"a"), "b", hasher_func);
    
    let iter = table.iter();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected.len(), 1);
    assert!(collected.contains(&&"b"));
}

#[test]
fn test_iter_multiple_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_func = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_func(&"a"), "b", hasher_func);
    table.insert_unique(hasher_func(&"b"), "c", hasher_func);
    table.insert_unique(hasher_func(&"c"), "d", hasher_func);
    
    let iter = table.iter();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected.len(), 3);
    assert!(collected.contains(&&"b"));
    assert!(collected.contains(&&"c"));
    assert!(collected.contains(&&"d"));
}

#[test]
fn test_iter_arbitrary_order() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_func = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher_func(&"a"), "b", hasher_func);
    table.insert_unique(hasher_func(&"b"), "c", hasher_func);
    table.insert_unique(hasher_func(&"c"), "d", hasher_func);

    let iter = table.iter();
    let collected: Vec<_> = iter.collect();
    let expected = vec![&"b", &"c", &"d"];
    
    for value in expected {
        assert!(collected.contains(value));
    }
    assert_eq!(collected.len(), expected.len());
}

