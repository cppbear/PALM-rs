// Answer 0

#[test]
fn test_get_mut_with_valid_entry() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    
    table.insert_unique(hasher.hash_one(&"test"), ("test", 1), |(k, _)| hasher.hash_one(k));

    if let OccupiedEntry { bucket, .. } = table.entry(hasher.hash_one(&"test"), |&(x, _)| x == "test", |(k, _)| hasher.hash_one(k)).unwrap() {
        let entry_mut = bucket.get_mut();
        *entry_mut = ("test", 10);
    }
}

#[test]
fn test_get_mut_with_multiple_insertions() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    
    for i in 1..=100 {
        table.insert_unique(hasher.hash_one(&format!("key{}", i)), (&format!("key{}", i), i), |(k, _)| hasher.hash_one(k));
    }

    if let OccupiedEntry { bucket, .. } = table.entry(hasher.hash_one(&"key50"), |&(x, _)| x == "key50", |(k, _)| hasher.hash_one(k)).unwrap() {
        let entry_mut = bucket.get_mut();
        entry_mut.1 += 5;
    }
}

#[test]
fn test_get_mut_panic_if_entry_does_not_exist() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    
    table.insert_unique(hasher.hash_one(&"existing_key"), ("existing_key", 1), |(k, _)| hasher.hash_one(k));
    
    let entry_panic = table.entry(hasher.hash_one(&"non_existing_key"), |&(x, _)| x == "non_existing_key", |(k, _)| hasher.hash_one(k));
    assert!(entry_panic.is_none());
}

