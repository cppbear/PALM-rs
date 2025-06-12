// Answer 0

#[test]
fn test_find_mut_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&1), (1, "a"), |val| hasher_fn(&val.0));
    if let Some(val) = table.find_mut(hasher_fn(&1), |val| val.0 == 1) {
        val.1 = "b";
    }
    assert_eq!(table.find(hasher_fn(&1), |val| val.0 == 1), Some(&(1, "b")));
}

#[test]
fn test_find_mut_non_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // No entry added, trying to find mutation should return None
    assert_eq!(table.find_mut(hasher_fn(&2), |val| val.0 == 2), None);
}

#[test]
fn test_find_mut_multiple_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&1), (1, "a"), |val| hasher_fn(&val.0));
    table.insert_unique(hasher_fn(&2), (2, "c"), |val| hasher_fn(&val.0));

    if let Some(val) = table.find_mut(hasher_fn(&1), |val| val.0 == 1) {
        val.1 = "b";
    }
    if let Some(val) = table.find_mut(hasher_fn(&2), |val| val.0 == 2) {
        val.1 = "d";
    }

    assert_eq!(table.find(hasher_fn(&1), |val| val.0 == 1), Some(&(1, "b")));
    assert_eq!(table.find(hasher_fn(&2), |val| val.0 == 2), Some(&(2, "d")));
}

#[should_panic]
#[test]
fn test_find_mut_panic_on_invalid_eq() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&1), (1, "a"), |val| hasher_fn(&val.0));

    // This should panic as we are trying to access an entry that doesn't exist correctly
    let _ = table.find_mut(hasher_fn(&1), |val| val.0 == 2);
}

