// Answer 0

#[test]
fn test_iter_empty() {
    use hashbrown::{HashTable, Global};

    let table: HashTable<i32, Global> = HashTable::new_in(Global);
    let iter = table.iter();
    assert!(iter.inner.items == 0);
}

#[test]
fn test_iter_single_element() {
    use hashbrown::{HashTable, Global};
    use std::hash::Hasher;

    let mut table: HashTable<&str, Global> = HashTable::new_in(Global);
    let hasher = |val: &&str| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(val.as_bytes());
        hasher.finish()
    };

    table.insert_unique(hasher(&"a"), "a", hasher);
    let mut iter = table.iter();
    assert!(iter.inner.items == 1);
    let first_item = unsafe { iter.inner.next() }.unwrap();
    assert_eq!(first_item, &"a");
}

#[test]
fn test_iter_multiple_elements() {
    use hashbrown::{HashTable, Global};
    use std::hash::Hasher;

    let mut table: HashTable<&str, Global> = HashTable::new_in(Global);
    let hasher = |val: &&str| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(val.as_bytes());
        hasher.finish()
    };

    table.insert_unique(hasher(&"a"), "a", hasher);
    table.insert_unique(hasher(&"b"), "b", hasher);
    table.insert_unique(hasher(&"c"), "c", hasher);

    let mut iter = table.iter();
    assert!(iter.inner.items == 3);
    
    let items: Vec<_> = unsafe { iter.inner.collect() };
    assert!(items.contains(&&"a"));
    assert!(items.contains(&&"b"));
    assert!(items.contains(&&"c"));
}

#[test]
fn test_iter_after_clear() {
    use hashbrown::{HashTable, Global};
    use std::hash::Hasher;

    let mut table: HashTable<&str, Global> = HashTable::new_in(Global);
    let hasher = |val: &&str| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(val.as_bytes());
        hasher.finish()
    };

    table.insert_unique(hasher(&"x"), "x", hasher);
    table.clear();

    let iter = table.iter();
    assert!(iter.inner.items == 0);
}

