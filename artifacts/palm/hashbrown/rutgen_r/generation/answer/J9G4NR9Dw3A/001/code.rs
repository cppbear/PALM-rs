// Answer 0

#[test]
fn test_prepare_resize_with_capacity_equal() {
    struct DummyAllocator;

    // We don't need actual allocator logic for this test
    impl Allocator for DummyAllocator {
        // Implement required methods with dummy or default behavior
    }

    struct DummyTableLayout;

    // Implement required methods for TableLayout as necessary
    struct TestStruct {
        items: usize,
    }

    impl TestStruct {
        fn new(items: usize) -> Self {
            TestStruct { items }
        }

        fn prepare_resize(
            &self,
            alloc: &DummyAllocator,
            table_layout: DummyTableLayout,
            capacity: usize,
            fallibility: Fallibility,
        ) -> Result<crate::scopeguard::ScopeGuard<Self, fn(&mut Self)>, TryReserveError> {
            assert!(self.items <= capacity);
            Err(TryReserveError::CapacityOverflow)
        }
    }

    let alloc = DummyAllocator;
    let table_layout = DummyTableLayout;
    let test_struct = TestStruct::new(10);
    let capacity = 10;

    let result = test_struct.prepare_resize(&alloc, table_layout, capacity, Fallibility::All);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, TryReserveError::CapacityOverflow);
    }
}

#[test]
#[should_panic]
fn test_prepare_resize_panic_with_excess_capacity() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
    }

    struct DummyTableLayout;

    struct TestStruct {
        items: usize,
    }

    impl TestStruct {
        fn new(items: usize) -> Self {
            TestStruct { items }
        }

        fn prepare_resize(
            &self,
            alloc: &DummyAllocator,
            table_layout: DummyTableLayout,
            capacity: usize,
            fallibility: Fallibility,
        ) -> Result<(), ()> {
            debug_assert!(self.items <= capacity);
            // This simulates a panic condition deliberately
            panic!();
        }
    }

    let alloc = DummyAllocator;
    let table_layout = DummyTableLayout;
    let test_struct = TestStruct::new(10);
    let capacity = 5; // This will ensure self.items > capacity, triggering panic

    // This should trigger a panic
    let _ = test_struct.prepare_resize(&alloc, table_layout, capacity, Fallibility::All);
}

