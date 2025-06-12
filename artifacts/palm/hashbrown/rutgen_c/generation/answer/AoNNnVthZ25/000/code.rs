// Answer 0

#[test]
fn test_new_in() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let bump = Bump::new();
    let mut table: HashTable<&str, Bump> = HashTable::new_in(&bump);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // The created HashTable holds none elements
    assert_eq!(table.len(), 0);
    
    // The created HashTable also doesn't allocate memory
    assert_eq!(table.capacity(), 0);
    
    // Now we insert element inside created HashTable
    table.insert_unique(hasher(&"One"), "One", hasher);
    // We can see that the HashTable holds 1 element
    assert_eq!(table.len(), 1);
    // And it also allocates some capacity
    assert!(table.capacity() > 0);
}

