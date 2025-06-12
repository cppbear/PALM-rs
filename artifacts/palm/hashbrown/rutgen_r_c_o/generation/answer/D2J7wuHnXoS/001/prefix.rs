// Answer 0

#[test]
fn test_get_many_unchecked_mut_valid_input() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &str| val.len() as u64;

    table.insert_unique(hasher("A"), ("A", 1), |(k, _)| hasher(k));
    table.insert_unique(hasher("B"), ("B", 2), |(k, _)| hasher(k));
    
    let keys = ["A", "B"];
    let results = unsafe {
        table.get_many_unchecked_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0)
    };
}

#[test]
fn test_get_many_unchecked_mut_missing_keys() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &str| val.len() as u64;

    table.insert_unique(hasher("A"), ("A", 1), |(k, _)| hasher(k));
    
    let keys = ["A", "C"];
    let results = unsafe {
        table.get_many_unchecked_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0)
    };
}

#[test]
fn test_get_many_unchecked_mut_edge_case_empty() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new_in(TestAllocator);
    
    let keys: [&str; 0] = [];
    let results: [Option<&'_ mut (&str, u32)>; 0] = unsafe {
        table.get_many_unchecked_mut([], |_, _| false)
    };
}

#[test]
fn test_get_many_unchecked_mut_edge_case_overlapping_hashes() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(&str, u32), TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &str| 1; // All hashes are the same

    table.insert_unique(hasher("A"), ("A", 1), |(k, _)| hasher(k));
    table.insert_unique(hasher("B"), ("B", 2), |(k, _)| hasher(k));
    
    let keys = ["A", "B"];
    let results = unsafe {
        table.get_many_unchecked_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0)
    };
}

