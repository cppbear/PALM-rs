// Answer 0

#[test]
fn test_entry_ref_debug_vacant() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let key: &str = "test_key";
    let hash: u64 = 12345;
    let table = &mut HashTable::<(_, ()), TestAllocator>::new();
    let vacant_entry = VacantEntryRef {
        hash,
        key,
        table,
    };

    let entry_ref = EntryRef::Vacant(vacant_entry);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry_ref);
    assert!(result.is_ok());
    assert!(output.contains("EntryRef"));
}

#[test]
fn test_entry_ref_debug_occupied() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let key: &str = "test_key_occupied";
    let value = 42;
    let hash: u64 = 67890;
    let table = &mut HashTable::<(_, ()), TestAllocator>::new();
    let occupied_entry = OccupiedEntry {
        hash,
        elem: Bucket::new(),
        table,
    };

    let entry_ref = EntryRef::Occupied(occupied_entry);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry_ref);
    assert!(result.is_ok());
    assert!(output.contains("EntryRef"));
}

