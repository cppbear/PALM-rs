// Answer 0

#[test]
fn test_new_uninitialized_with_buckets_not_power_of_two() {
    let alloc = std::alloc::System; // Default system allocator
    let bucket_count = 3; // Not a power of two
    let fallibility = Fallibility::default();
    let table_layout = TableLayout::default(); // Default table layout
    
    // This will trigger an error due to buckets not being a power of two
    let result = unsafe { new_uninitialized(&alloc, table_layout, bucket_count, fallibility) };
    
    assert_eq!(result, Err(fallibility.capacity_overflow()));
}

#[test]
fn test_new_uninitialized_with_capacity_overflow() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Mock implementations for required methods
    }

    struct MockTableLayout;

    impl MockTableLayout {
        fn calculate_layout_for(&self, buckets: usize) -> Option<(Layout, usize)> {
            None // Return None to trigger capacity overflow
        }
    }

    // Using small power-of-two value for buckets
    let alloc = MockAllocator;
    let buckets = 4; // A power of two
    let fallibility = Fallibility::default();
    let table_layout = MockTableLayout; // Use mock layout

    // This should return Err since calculate_layout_for returns None
    let result = unsafe { new_uninitialized(&alloc, table_layout, buckets, fallibility) };

    assert_eq!(result, Err(fallibility.capacity_overflow()));
}

