// Answer 0

#[cfg(test)]
fn test_with_capacity_in() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let bump = Bump::new();
    let mut table = HashTable::with_capacity_in(5, &bump);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // The created HashTable holds no elements
    assert_eq!(table.len(), 0);
    // But it can hold at least 5 elements without reallocating
    let empty_map_capacity = table.capacity();
    assert!(empty_map_capacity >= 5);

    // Now we insert some 5 elements inside created HashTable
    table.insert_unique(hasher(&"One"), "One", hasher);
    table.insert_unique(hasher(&"Two"), "Two", hasher);
    table.insert_unique(hasher(&"Three"), "Three", hasher);
    table.insert_unique(hasher(&"Four"), "Four", hasher);
    table.insert_unique(hasher(&"Five"), "Five", hasher);

    // We can see that the HashTable holds 5 elements
    assert_eq!(table.len(), 5);
    // But its capacity isn't changed
    assert_eq!(table.capacity(), empty_map_capacity);
}

#[test]
fn test_create_empty_table() {
    test_with_capacity_in();
}

#[test]
#[should_panic]
fn test_create_empty_table_zero_capacity() {
    let bump = Bump::new();
    let table = HashTable::with_capacity_in(0, &bump);
    assert_eq!(table.len(), 0);
}

