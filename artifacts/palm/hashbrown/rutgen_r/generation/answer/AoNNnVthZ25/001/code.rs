// Answer 0

#[test]
fn test_create_empty_hashtable() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let bump = Bump::new();
    let table: HashTable<&str, &str, DefaultHashBuilder> = HashTable::new_in(&bump);

    // The created HashTable holds no elements
    assert_eq!(table.len(), 0);

    // The created HashTable also doesn't allocate memory
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_insert_into_hashtable() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let bump = Bump::new();
    let mut table = HashTable::new_in(&bump);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert element inside created HashTable
    table.insert_unique(hasher(&"One"), "One", hasher);
    
    // Now we can see that the HashTable holds 1 element
    assert_eq!(table.len(), 1);
    
    // And it also allocates some capacity
    assert!(table.capacity() > 0);
}

#[test]
#[should_panic]
fn test_insert_with_invalid_key() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let bump = Bump::new();
    let mut table = HashTable::new_in(&bump);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Intentionally trying to insert invalid key
    table.insert_unique(hasher(&"Invalid_Key"), "SomeValue", hasher);
}

#[test]
fn test_multiple_insertions() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let bump = Bump::new();
    let mut table = HashTable::new_in(&bump);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Inserting multiple elements
    table.insert_unique(hasher(&"One"), "One", hasher);
    table.insert_unique(hasher(&"Two"), "Two", hasher);
    
    // Now we can see that the HashTable holds 2 elements
    assert_eq!(table.len(), 2);

    // And the capacity must have increased
    assert!(table.capacity() > 2);
}

