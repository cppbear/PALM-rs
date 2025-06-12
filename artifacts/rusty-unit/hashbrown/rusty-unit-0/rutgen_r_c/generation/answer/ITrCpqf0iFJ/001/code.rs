// Answer 0

#[test]
fn test_iter_hash_mut() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Inserting elements to create multiple entries for a single hash
    let hashes = [hasher(&1), hasher(&1), hasher(&2)];
    table.insert_unique(hashes[0], 2, hasher);
    table.insert_unique(hashes[0], 3, hasher);
    table.insert_unique(hashes[1], 5, hasher);

    // Ensure that the iterator can visit elements corresponding to a single hash
    let mut iter = table.iter_hash_mut(hashes[0]);
    let mut values: Vec<i32> = Vec::new();

    for val in &mut iter {
        *val *= 2; // Update matching values
        values.push(*val);
    }

    assert_eq!(table.len(), 3); // Ensure the number of elements remains the same
    assert_eq!(values.sort(), vec![4, 6]); // Check if the updated values are as expected
    
    // Ensure the original values remained the same outside the iterator
    let mut original_values: Vec<i32> = Vec::new();
    for val in &table {
        original_values.push(*val);
    }
    
    assert!(original_values.contains(&4)); // Check for updated values
    assert!(original_values.contains(&6)); // Check for updated values
    assert!(original_values.contains(&5)); // Ensure the unmodified value is present
}

#[test]
#[should_panic]
fn test_iter_hash_mut_panic() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Attempting to iterate over an empty table should not panic
    let mut iter = table.iter_hash_mut(0);
    for _val in &mut iter {
        // This block should not execute because the table is empty
    }
}

