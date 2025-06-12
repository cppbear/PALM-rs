// Answer 0

#[test]
fn test_clear_no_drop_empty_singleton() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary allocator methods here
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default();
    let mut inner = RawTableInner::with_capacity(&alloc, table_layout, 0);

    inner.clear_no_drop();

    assert_eq!(inner.items, 0);
    assert_eq!(inner.growth_left, bucket_mask_to_capacity(inner.bucket_mask));
}

#[test]
fn test_clear_no_drop_non_empty() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary allocator methods here
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default();
    let mut inner = RawTableInner::with_capacity(&alloc, table_layout, 16); // More than zero capacity
    inner.items = 5; // Simulate that the table has some items

    unsafe {
        inner.ctrl(0).write_bytes(Tag::EMPTY.0, inner.num_ctrl_bytes());
    }

    inner.clear_no_drop();

    assert_eq!(inner.items, 0);
    assert_eq!(inner.growth_left, bucket_mask_to_capacity(inner.bucket_mask));
}

