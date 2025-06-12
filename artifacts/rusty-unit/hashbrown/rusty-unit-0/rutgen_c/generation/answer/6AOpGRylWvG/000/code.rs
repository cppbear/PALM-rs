// Answer 0

#[test]
fn test_with_capacity_zero() {
    struct AllocatorStruct;

    impl Allocator for AllocatorStruct {
        // Implement required methods for Allocator trait
    }

    let allocator = AllocatorStruct;
    let table_layout = TableLayout { size: 0, ctrl_align: 0 };

    let table_inner = RawTableInner::with_capacity(&allocator, table_layout, 0);
    assert_eq!(table_inner.buckets(), 0);
}

#[test]
fn test_with_capacity_small() {
    struct AllocatorStruct;

    impl Allocator for AllocatorStruct {
        // Implement required methods for Allocator trait
    }

    let allocator = AllocatorStruct;
    let table_layout = TableLayout { size: 4, ctrl_align: 8 };

    let table_inner = RawTableInner::with_capacity(&allocator, table_layout, 2);
    assert!(table_inner.buckets() >= 2);
}

#[test]
#[should_panic]
fn test_with_capacity_exceeds_maximum() {
    struct AllocatorStruct;

    impl Allocator for AllocatorStruct {
        // Implement required methods for Allocator trait
    }

    let allocator = AllocatorStruct;
    let table_layout = TableLayout { size: usize::MAX, ctrl_align: 8 };

    // Attempting to allocate with a capacity that would overflow
    RawTableInner::with_capacity(&allocator, table_layout, usize::MAX);
} 

#[test]
fn test_with_capacity_buckets_power_of_two() {
    struct AllocatorStruct;

    impl Allocator for AllocatorStruct {
        // Implement required methods for Allocator trait
    }

    let allocator = AllocatorStruct;
    let table_layout = TableLayout { size: 4, ctrl_align: 8 };

    let table_inner = RawTableInner::with_capacity(&allocator, table_layout, 8);
    assert!(table_inner.buckets().is_power_of_two());
}

