// Answer 0

#[test]
fn test_new_uninitialized_success() {
    use std::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implementation would go here if needed, but let's keep it minimal.
    }

    let alloc = DummyAllocator;
    let fallibility = Fallibility::default();
    let table_layout = TableLayout::default();
    let buckets = 16; // A power of two

    let result = unsafe {
        new_uninitialized(&alloc, table_layout, buckets, fallibility)
    };

    assert!(result.is_ok());
    let table = result.unwrap();
    assert_eq!(table.bucket_mask, buckets - 1);
    assert_eq!(table.items, 0);
    assert_eq!(table.growth_left, bucket_mask_to_capacity(buckets - 1));
}

#[test]
#[should_panic]
fn test_new_uninitialized_not_power_of_two() {
    use std::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implementation would go here if needed, but let's keep it minimal.
    }

    let alloc = DummyAllocator;
    let fallibility = Fallibility::default();
    let table_layout = TableLayout::default();
    let buckets = 15; // Not a power of two

    let _ = unsafe {
        new_uninitialized(&alloc, table_layout, buckets, fallibility)
    };
}

#[test]
fn test_new_uninitialized_capacity_overflow() {
    use std::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implementation would go here if needed, but let's keep it minimal.
    }

    let alloc = DummyAllocator;
    let fallibility = Fallibility::default();
    let table_layout = TableLayout::default();
    let buckets = 1 << 30; // A large power of two that might cause overflow

    let result = unsafe {
        new_uninitialized(&alloc, table_layout, buckets, fallibility)
    };

    assert!(result.is_err());
    assert_eq!(result.err(), Some(fallibility.capacity_overflow()));
}

