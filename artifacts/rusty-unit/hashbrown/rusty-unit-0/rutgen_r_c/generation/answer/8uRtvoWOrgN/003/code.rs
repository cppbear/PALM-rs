// Answer 0

#[test]
fn test_get_many_mut_no_duplicates() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(4, TestAllocator);

    // Insert some values into the table
    let hashes = [1, 2, 3];
    for &hash in &hashes {
        table.insert(hash, hash * 10, |&x| x);
    }

    // Retrieve mutable references to the entries
    let results = table.get_many_mut(hashes, |i, &k| k == hash);
    
    // Check that we received mutable references
    for (i, res) in results.iter().enumerate() {
        assert!(res.is_some(), "Expected value at index {} to be Some", i);
        assert_eq!(res.unwrap(), &mut (hashes[i] * 10));
    }
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_duplicates() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(4, TestAllocator);

    // Insert some values into the table
    let hashes = [1, 2];
    for &hash in &hashes {
        table.insert(hash, hash * 10, |&x| x);
    }

    // Attempt to retrieve with a duplicate
    let duplicate_hashes = [1, 1]; // Duplicate hash
    table.get_many_mut(duplicate_hashes, |i, &k| k == hash);
}

#[test]
fn test_get_many_mut_not_found() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(4, TestAllocator);

    // Insert some values into the table
    let hashes = [1, 2, 3];
    for &hash in &hashes {
        table.insert(hash, hash * 10, |&x| x);
    }

    // Retrieve mutable references to non-existing entries
    let non_existing_hashes = [4, 5, 6]; // These hashes do not exist in the table
    let results = table.get_many_mut(non_existing_hashes, |i, &k| k == hash);

    // Check that we received None for all
    for (i, res) in results.iter().enumerate() {
        assert!(res.is_none(), "Expected value at index {} to be None", i);
    }
}

