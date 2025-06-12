// Answer 0

#[test]
fn test_is_in_same_group_true() {
    struct TestAllocator;
    struct TestTableLayout;
    struct TestFallibility;

    impl Allocator for TestAllocator {}
    impl TableLayout for TestTableLayout {
        fn calculate_layout_for(&self, _buckets: usize) -> Option<(Layout, usize)> {
            Some((Layout::from_size_align(0, 1).unwrap(), 0))
        }
    }
    impl Fallibility for TestFallibility {
        fn capacity_overflow(&self) {}
        fn alloc_err(&self, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let table_layout = TestTableLayout;
    let fallibility = TestFallibility;

    let raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, 8);
    
    let hash = 12345;
    let i = 5;
    let new_i = 4;

    assert!(raw_table_inner.is_in_same_group(i, new_i, hash));
}

#[test]
fn test_is_in_same_group_false() {
    struct TestAllocator;
    struct TestTableLayout;
    struct TestFallibility;

    impl Allocator for TestAllocator {}
    impl TableLayout for TestTableLayout {
        fn calculate_layout_for(&self, _buckets: usize) -> Option<(Layout, usize)> {
            Some((Layout::from_size_align(0, 1).unwrap(), 0))
        }
    }
    impl Fallibility for TestFallibility {
        fn capacity_overflow(&self) {}
        fn alloc_err(&self, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let table_layout = TestTableLayout;
    let fallibility = TestFallibility;

    let raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, 8);
    
    let hash = 12345;
    let i = 5;
    let new_i = 8;

    assert!(!raw_table_inner.is_in_same_group(i, new_i, hash));
}

