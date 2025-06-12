// Answer 0

#[test]
fn test_prepare_resize_items_greater_than_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator { /* implementation here */ }

    let allocator = TestAllocator;
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let items = 10;
    let capacity = 5;

    let raw_table_inner = RawTableInner {
        bucket_mask: 0b1111,
        ctrl: NonNull::new_unchecked(ptr::null_mut()),
        growth_left: 16,
        items,
    };

    let result = raw_table_inner.prepare_resize(&allocator, table_layout, capacity, Fallibility::Fallible);
}

#[test]
#[should_panic]
fn test_prepare_resize_items_equal_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator { /* implementation here */ }

    let allocator = TestAllocator;
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let items = 10;
    let capacity = 10;

    let raw_table_inner = RawTableInner {
        bucket_mask: 0b1111,
        ctrl: NonNull::new_unchecked(ptr::null_mut()),
        growth_left: 16,
        items,
    };

    let result = raw_table_inner.prepare_resize(&allocator, table_layout, capacity, Fallibility::Fallible);
}

#[test]
fn test_prepare_resize_capacity_zero() {
    struct TestAllocator;
    impl Allocator for TestAllocator { /* implementation here */ }

    let allocator = TestAllocator;
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    let items = 1;
    let capacity = 0; // This should technically not panic as items can be greater

    let raw_table_inner = RawTableInner {
        bucket_mask: 0b1111,
        ctrl: NonNull::new_unchecked(ptr::null_mut()),
        growth_left: 16,
        items,
    };

    let result = raw_table_inner.prepare_resize(&allocator, table_layout, capacity, Fallibility::Fallible);
}

