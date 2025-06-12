// Answer 0

#[test]
fn test_find_inner_empty_bucket() {
    struct MockAllocator;
    struct MockTableLayout;
    struct MockFallibility;

    let allocator = MockAllocator;
    let table_layout = MockTableLayout;
    let fallibility = MockFallibility;

    let mut raw_table = unsafe {
        RawTableInner::new_uninitialized(&allocator, table_layout, 4, fallibility)
            .expect("Failed to create RawTableInner")
    };

    let hash: u64 = 42;
    let mut found_index = false;

    let eq_fn = &mut |index: usize| {
        found_index = true;
        index == 0 // Simulate that index 0 matches
    };

    unsafe {
        raw_table.find_inner(hash, eq_fn);
    }

    assert!(found_index);
}

#[test]
fn test_find_inner_no_match() {
    struct MockAllocator;
    struct MockTableLayout;
    struct MockFallibility;

    let allocator = MockAllocator;
    let table_layout = MockTableLayout;
    let fallibility = MockFallibility;

    let mut raw_table = unsafe {
        RawTableInner::new_uninitialized(&allocator, table_layout, 4, fallibility)
            .expect("Failed to create RawTableInner")
    };

    let hash: u64 = 99;
    let mut attempts = 0;

    let eq_fn = &mut |index: usize| {
        attempts += 1;
        false // Simulate no match for any index
    };

    unsafe {
        let result = raw_table.find_inner(hash, eq_fn);
        assert!(result.is_none());
    }

    assert_eq!(attempts, 4); // All indices were attempted
}

#[test]
#[should_panic]
fn test_find_inner_infinite_loop() {
    struct MockAllocator;
    struct MockTableLayout;
    struct MockFallibility;

    let allocator = MockAllocator;
    let table_layout = MockTableLayout;
    let fallibility = MockFallibility;

    let mut raw_table = unsafe {
        RawTableInner::new_uninitialized(&allocator, table_layout, 1, fallibility)
            .expect("Failed to create RawTableInner")
    };

    let hash: u64 = 100;
    let eq_fn = &mut |_| true; // Always returns true

    unsafe {
        raw_table.find_inner(hash, eq_fn); // This should panic due to infinite loop
    }
}

