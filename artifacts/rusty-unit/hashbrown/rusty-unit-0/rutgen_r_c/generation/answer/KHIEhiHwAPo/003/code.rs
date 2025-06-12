// Answer 0

#[test]
fn test_new_uninitialized_power_of_two_buckets_overflow() {
    use crate::alloc::Global;

    struct DummyAllocator;

    // Implementing the Allocator trait minimally for the test
    impl Allocator for DummyAllocator {
        // body needs to handle allocation
        // Placeholders to satisfy the trait
    }

    let allocator = DummyAllocator;
    let layout = TableLayout::new::<u8>();
    
    // Test with a buckets value that, when passed to calculate_layout_for,
    // should result in None (e.g., capacity exceeding max).
    let buckets: usize = 1 << 30; // A very high power of two

    let result = unsafe {
        RawTableInner::new_uninitialized(&allocator, layout, buckets, Fallibility::Fallible)
    };

    assert_eq!(result, Err(TryReserveError::CapacityOverflow));
}

#[test]
fn test_new_uninitialized_not_power_of_two_buckets() {
    use crate::alloc::Global;

    struct DummyAllocator;

    // Implementing the Allocator trait minimally for the test
    impl Allocator for DummyAllocator {
        // body needs to handle allocation
        // Placeholders to satisfy the trait
    }

    let allocator = DummyAllocator;
    let layout = TableLayout::new::<u8>();
    
    // Use a non-power of two value
    let buckets: usize = 3; // Not a power of 2

    let result = unsafe {
        RawTableInner::new_uninitialized(&allocator, layout, buckets, Fallibility::Fallible)
    };

    // Since buckets is not a power of two, it should also panic
    assert_eq!(result, Err(TryReserveError::CapacityOverflow));
}

