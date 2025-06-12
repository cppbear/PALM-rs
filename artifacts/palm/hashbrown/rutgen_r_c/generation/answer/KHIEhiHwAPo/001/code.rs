// Answer 0

#[test]
fn test_new_uninitialized_power_of_two_buckets_success() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement required methods for the Allocator trait
    }

    let alloc = TestAllocator;
    let layout = TableLayout::new::<u8>();
    let buckets = 16; // 16 is a power of two
    let fallibility = Fallibility::Fallible;

    let result = unsafe { RawTableInner::new_uninitialized(&alloc, layout, buckets, fallibility) };

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_new_uninitialized_power_of_two_buckets_failure() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(()) // Force allocation error
        }
    }

    let alloc = TestAllocator;
    let layout = TableLayout::new::<u8>();
    let buckets = 16; // 16 is a power of two
    let fallibility = Fallibility::Fallible;

    let result = unsafe { RawTableInner::new_uninitialized(&alloc, layout, buckets, fallibility) };

    match result {
        Ok(_) => panic!("Expected an error, but got Ok"),
        Err(err) => match err {
            TryReserveError::AllocError { layout: _ } => {}
            _ => panic!("Expected AllocError, but got {:?}", err),
        },
    }
}

#[test]
fn test_new_uninitialized_non_power_of_two_buckets() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement required methods for the Allocator trait
    }

    let alloc = TestAllocator;
    let layout = TableLayout::new::<u8>();
    let buckets = 10; // 10 is not a power of two
    let fallibility = Fallibility::Fallible;

    let result = unsafe { RawTableInner::new_uninitialized(&alloc, layout, buckets, fallibility) };

    assert!(result.is_err());
}

#[test]
fn test_new_uninitialized_capacity_overflow() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(()) // Force allocation error
        }
    }

    let alloc = TestAllocator;
    let layout = TableLayout::new::<u8>();
    let buckets = usize::MAX; // Test with max usize to cause capacity overflow
    let fallibility = Fallibility::Fallible;

    let result = unsafe { RawTableInner::new_uninitialized(&alloc, layout, buckets, fallibility) };

    match result {
        Ok(_) => panic!("Expected an error, but got Ok"),
        Err(err) => match err {
            TryReserveError::CapacityOverflow => {}
            _ => panic!("Expected CapacityOverflow, but got {:?}", err),
        },
    }
}

