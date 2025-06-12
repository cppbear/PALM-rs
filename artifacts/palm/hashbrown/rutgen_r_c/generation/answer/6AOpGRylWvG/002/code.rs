// Answer 0

#[test]
fn test_with_capacity_normal_case() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement required methods for the allocator here
    }

    let allocator = MockAllocator;
    let layout = TableLayout { size: 32, ctrl_align: 8 };
    let capacity = 16;  // A valid capacity

    let table_inner = RawTableInner::with_capacity(&allocator, layout, capacity);
    assert_eq!(table_inner.buckets(), capacity_to_buckets(capacity));
}

#[test]
#[should_panic]
fn test_with_capacity_overflow() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement required methods for the allocator here
    }

    let allocator = MockAllocator;
    let layout = TableLayout { size: 32, ctrl_align: 8 };
    let capacity = usize::MAX; // This should cause a panic due to overflow

    RawTableInner::with_capacity(&allocator, layout, capacity);
}

#[test]
fn test_with_capacity_zero_capacity() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement required methods for the allocator here
    }

    let allocator = MockAllocator;
    let layout = TableLayout { size: 32, ctrl_align: 8 };
    let capacity = 0; // Zero capacity should work

    let table_inner = RawTableInner::with_capacity(&allocator, layout, capacity);
    assert_eq!(table_inner.buckets(), capacity_to_buckets(capacity));
}

