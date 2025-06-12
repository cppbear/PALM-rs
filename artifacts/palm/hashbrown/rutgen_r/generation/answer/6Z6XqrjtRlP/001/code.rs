// Answer 0

#[test]
fn test_with_capacity_in_non_zero_capacity() {
    use bumpalo::Bump;
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let bump = Bump::new();
    let mut table = HashTable::with_capacity_in(5, &bump);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    assert_eq!(table.len(), 0);
    let empty_map_capacity = table.capacity();
    assert!(empty_map_capacity >= 5);

    table.insert_unique(hasher(&"One"), "One", hasher);
    table.insert_unique(hasher(&"Two"), "Two", hasher);
    table.insert_unique(hasher(&"Three"), "Three", hasher);
    table.insert_unique(hasher(&"Four"), "Four", hasher);
    table.insert_unique(hasher(&"Five"), "Five", hasher);

    assert_eq!(table.len(), 5);
    assert_eq!(table.capacity(), empty_map_capacity);
}

#[test]
fn test_with_capacity_in_zero_capacity() {
    use bumpalo::Bump;
    use hashbrown::{HashTable};
    
    let bump = Bump::new();
    let table = HashTable::with_capacity_in(0, &bump);

    assert_eq!(table.len(), 0);
    assert_eq!(table.capacity(), 0);
}

#[test]
#[should_panic]
fn test_with_capacity_in_large_capacity() {
    use bumpalo::Bump;
    use hashbrown::{HashTable};

    let bump = Bump::new();
    let _table = HashTable::with_capacity_in(usize::MAX, &bump);
}

