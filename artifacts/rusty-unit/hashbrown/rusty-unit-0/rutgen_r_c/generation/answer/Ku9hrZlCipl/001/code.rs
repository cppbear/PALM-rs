// Answer 0

#[test]
fn test_shrink_to_fit() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a HashTable with an initial capacity of 100
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Insert unique values into the table
    table.insert_unique(hasher_fn(&1), 1, hasher_fn);
    table.insert_unique(hasher_fn(&2), 2, hasher_fn);
    assert!(table.capacity() >= 100); // Ensure the capacity is initially as expected

    table.shrink_to_fit(hasher_fn); // Call the function under test
    assert!(table.capacity() >= 2); // Verify that the capacity shrinks to at least 2
}

#[test]
fn test_shrink_to_fit_empty() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create an empty HashTable
    let mut table = HashTable::<i32>::new_in(Global);
    assert!(table.capacity() == 0); // Initial capacity should be 0

    // Attempt to shrink to fit (which should be a no-op)
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    table.shrink_to_fit(hasher_fn); 

    assert!(table.capacity() == 0); // Verify that the capacity remains 0
}

#[test]
fn test_shrink_to_fit_with_one_element() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a HashTable with an initial capacity of 10
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Insert a single element
    table.insert_unique(hasher_fn(&1), 1, hasher_fn);
    assert!(table.capacity() >= 10); // Ensure the capacity is as expected

    // Shrink the table to fit
    table.shrink_to_fit(hasher_fn);
    assert!(table.capacity() >= 1); // Verify that the capacity shrinks to at least 1
}

