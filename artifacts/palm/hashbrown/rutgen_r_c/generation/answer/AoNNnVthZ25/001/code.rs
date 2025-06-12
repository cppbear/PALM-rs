// Answer 0

#[test]
fn test_hash_table_new_in() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, Global};

    // Create a new Bump allocator
    let bump = Bump::new();
    
    // Initialize the HashTable using the Bump allocator
    let table: HashTable<i32, Global> = HashTable::new_in(&bump);
    
    // Assert that the table is empty
    assert_eq!(table.len(), 0);
    
    // Assert that the capacity is 0 as expected since it hasn't allocated yet
    assert_eq!(table.capacity(), 0);
}

#[test]
fn test_hash_table_new_in_with_capacity() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, Global};

    // Create a new Bump allocator
    let bump = Bump::new();
    
    // Create a HashTable with some initial capacity
    let mut table = HashTable::with_capacity_in(10, &bump);
    
    // Assert that the table is empty initially
    assert_eq!(table.len(), 0);
    
    // Assert that the capacity is greater than or equal to 10
    assert!(table.capacity() >= 10);
}

#[test]
fn test_hash_table_insert_unique() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    // Create a new Bump allocator
    let bump = Bump::new();
    let mut table = HashTable::new_in(&bump);
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &str| hasher.hash_one(val);
    
    // Insert an element into the table
    table.insert_unique(hasher_fn("One"), "One", hasher_fn);
    
    // Assert that the table now contains one element
    assert_eq!(table.len(), 1);
    
    // Assert that the capacity is greater than 0 after insertion
    assert!(table.capacity() > 0);
}

