// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"test_key"), "initial_value", hasher);
    
    let entry = table
        .entry(hasher(&"test_key"), |&x| x == "test_key", hasher)
        .insert("new_value");
}

#[test]
fn test_insert_another_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<u32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 42, hasher);
    
    let entry = table
        .entry(hasher(&1), |&x| x == &1, hasher)
        .insert(84);
}

#[test]
fn test_insert_multiple_occupied_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<f64> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    for i in 0..10 {
        table.insert_unique(hasher(&(i as f64)), i as f64, hasher);
    }

    for i in 0..10 {
        let entry = table
            .entry(hasher(&(i as f64)), |&x| x == &(i as f64), hasher)
            .insert((i + 1) as f64);
    }
}

#[test]
#[should_panic]
fn test_insert_panic_empty_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table
        .entry(hasher(&"non_existent_key"), |&x| x == "non_existent_key", hasher)
        .insert("value");
}

