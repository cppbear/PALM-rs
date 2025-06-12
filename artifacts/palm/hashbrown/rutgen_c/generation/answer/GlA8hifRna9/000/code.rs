// Answer 0

#[test]
fn test_find_entry_found() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(u64, &str), TestAllocator> = HashTable::with_capacity_in(1, TestAllocator);
    let hasher = |val: &(u64, &str)| val.0;
    table.insert_unique(1, (1, "a"), |val| hasher(val));

    let result = table.find_entry(1, |val| val.0 == 1);
    assert!(result.is_ok());
    let entry = result.unwrap();
    assert_eq!(entry.bucket.ptr.as_ref(), &(1, "a"));
}

#[test]
fn test_find_entry_not_found() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(u64, &str), TestAllocator> = HashTable::with_capacity_in(1, TestAllocator);

    let result = table.find_entry(1, |val| val.0 == 1);
    assert!(result.is_err());
}

#[test]
fn test_find_entry_multiple_calls() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<(u64, &str), TestAllocator> = HashTable::with_capacity_in(2, TestAllocator);
    let hasher = |val: &(u64, &str)| val.0;

    table.insert_unique(1, (1, "a"), |val| hasher(val));
    table.insert_unique(2, (2, "b"), |val| hasher(val));

    let result_a = table.find_entry(1, |val| val.0 == 1);
    assert!(result_a.is_ok());

    let result_b = table.find_entry(2, |val| val.0 == 2);
    assert!(result_b.is_ok());

    let result_c = table.find_entry(1, |val| val.0 == 2);
    assert!(result_c.is_err());
}

