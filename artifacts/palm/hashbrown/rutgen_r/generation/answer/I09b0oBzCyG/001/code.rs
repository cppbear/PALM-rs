// Answer 0

#[test]
fn test_len_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let v = HashTable::new();
    assert_eq!(v.len(), 0);
}

#[test]
fn test_len_single_insertion() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let mut v = HashTable::new();
    v.insert_unique(hasher(&1), 1, hasher);
    assert_eq!(v.len(), 1);
}

#[test]
fn test_len_multiple_insertions() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let mut v = HashTable::new();
    for i in 0..10 {
        v.insert_unique(hasher(&i), i, hasher);
    }
    assert_eq!(v.len(), 10);
}

#[test]
fn test_len_removal() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let mut v = HashTable::new();
    for i in 0..5 {
        v.insert_unique(hasher(&i), i, hasher);
    }
    assert_eq!(v.len(), 5);

    for i in 0..5 {
        v.remove(&hasher(&i));
    }
    assert_eq!(v.len(), 0);
}

#[test]
fn test_len_after_reinsertion() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let mut v = HashTable::new();
    v.insert_unique(hasher(&1), 1, hasher);
    v.insert_unique(hasher(&1), 2, hasher); // Updating key
    assert_eq!(v.len(), 1); // Ensure length remains 1 after updating
}

