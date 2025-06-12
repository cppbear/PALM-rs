// Answer 0

#[test]
fn test_get_many_mut_pointers_with_present_hashes() {
    struct SimpleAllocator;
    
    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut raw_table: RawTable<u64, SimpleAllocator> = RawTable::new_in(SimpleAllocator);
    
    // Simulate inserting elements
    let hashes = [1, 2, 3, 4];
    for &hash in &hashes {
        raw_table.insert(hash, hash, |&value| value);
    }

    // Function to test equality
    let eq = |i: usize, &k: &u64| {
        hashes[i] == k
    };

    // Call the function under test
    unsafe {
        let results = raw_table.get_many_mut_pointers(hashes, eq);

        // Check that each result matches the expected
        assert_eq!(results.len(), hashes.len());
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_some()); // Expecting some non-null pointers
            assert_eq!(result.unwrap().as_ref(), &hashes[i]);
        }
    }
}

#[test]
fn test_get_many_mut_pointers_with_absent_hashes() {
    struct SimpleAllocator;
    
    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut raw_table: RawTable<u64, SimpleAllocator> = RawTable::new_in(SimpleAllocator);
    
    // Simulate inserting elements
    raw_table.insert(1, 1, |&value| value);
    raw_table.insert(2, 2, |&value| value);

    // Use hashes that are not present in the raw table
    let hashes = [3, 4, 5, 6];
    
    // Function to test equality
    let eq = |i: usize, &k: &u64| {
        hashes[i] == k
    };

    // Call the function under test
    unsafe {
        let results = raw_table.get_many_mut_pointers(hashes, eq);

        // Check that all results are None since the hashes are absent
        assert_eq!(results.len(), hashes.len());
        for result in results.iter() {
            assert!(result.is_none()); // Expecting None
        }
    }
}

#[test]
fn test_get_many_mut_pointers_with_mixed_hashes() {
    struct SimpleAllocator;
    
    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut raw_table: RawTable<u64, SimpleAllocator> = RawTable::new_in(SimpleAllocator);
    
    // Simulate inserting elements
    raw_table.insert(1, 1, |&value| value);
    raw_table.insert(3, 3, |&value| value);

    // Use a mix of present and absent hashes
    let hashes = [1, 2, 3, 4];
    
    // Function to test equality
    let eq = |i: usize, &k: &u64| {
        let present_hashes = [1, 3];
        present_hashes.contains(&k) && (hashes[i] == k)
    };

    // Call the function under test
    unsafe {
        let results = raw_table.get_many_mut_pointers(hashes, eq);

        // Check results for presence and absence
        assert_eq!(results.len(), hashes.len());
        
        assert!(results[0].is_some()); // 1 is present
        assert!(results[1].is_none());  // 2 is absent
        assert!(results[2].is_some()); // 3 is present
        assert!(results[3].is_none());  // 4 is absent
    }
}

