// Answer 0

#[test]
fn test_find_mut_existing_entry() {
    struct Allocator;
    
    impl Allocator {
        fn new() -> Self {
            Allocator
        }
    }
    
    let mut table: HashTable<(u64, &str), Allocator> = HashTable::with_capacity_in(10, Allocator::new());
    
    // Inserting a unique entry
    let hasher = |val: &(u64, &str)| val.0;
    let hash_value = hasher(&(1, "a"));
    table.insert_unique(hash_value, (1, "a"), &hasher);
    
    // Finding and mutating the entry
    if let Some(val) = table.find_mut(hash_value, |val| val.0 == 1) {
        val.1 = "b";
    }
    
    // Verifying the mutation
    assert_eq!(table.find(hash_value, |val| val.0 == 1), Some(&(1, "b")));
}

#[test]
fn test_find_mut_non_existent_entry() {
    struct Allocator;
    
    impl Allocator {
        fn new() -> Self {
            Allocator
        }
    }
    
    let mut table: HashTable<(u64, &str), Allocator> = HashTable::with_capacity_in(10, Allocator::new());

    // Inserting an entry
    let hasher = |val: &(u64, &str)| val.0;
    table.insert_unique(hasher(&(1, "a")), (1, "a"), &hasher);
    
    // Trying to find a non-existent entry
    let result = table.find_mut(2, |val| val.0 == 2);
    assert!(result.is_none());
}

