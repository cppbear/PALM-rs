// Answer 0

#[test]
fn test_iter_hash_basic() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {}

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &str| {
        val.len() as u64 // Simple example hash based on string length
    };

    table.insert_unique(hasher("a"), "a", hasher);
    table.insert_unique(hasher("b"), "b", hasher);
    table.insert_unique(hasher("b"), "c", hasher);

    let mut iter = table.iter_hash(hasher("a"));
    let mut found = vec![];

    while let Some(value) = iter.next() {
        found.push(value);
    }

    assert_eq!(found.contains(&&"a"), true);
    assert_eq!(found.contains(&&"b"), true);
    assert_eq!(found.contains(&&"c"), true);
}

#[test]
fn test_iter_hash_empty() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {}

    let table: HashTable<i32, DummyAllocator> = HashTable::new_in(DummyAllocator);
    let mut iter = table.iter_hash(0);
    
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_hash_single_element() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {}

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &i32| *val as u64;

    table.insert_unique(hasher(&10), 10, hasher);

    let mut iter = table.iter_hash(hasher(&10));
    
    assert_eq!(iter.next(), Some(&10));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_hash_multiple_same_hash() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {}

    let mut table = HashTable::new_in(DummyAllocator);
    let hasher = |val: &str| {
        val.len() as u64
    };

    table.insert_unique(hasher("a"), "a", hasher);
    table.insert_unique(hasher("b"), "b", hasher);
    table.insert_unique(hasher("c"), "b", hasher); // "b" should be inserted again

    let mut iter = table.iter_hash(hasher("b"));

    let mut found = vec![];
    while let Some(value) = iter.next() {
        found.push(value);
    }

    assert!(found.len() >= 2); // Expecting at least two entries since "b" was inserted twice.
}

