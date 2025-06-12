// Answer 0

#[test]
fn test_allocation_size_empty() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    assert_eq!(map.allocation_size(), 0);
}

#[test]
fn test_allocation_size_with_elements() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, 10);
    map.insert(2, 20);

    // Assuming DummyAllocator simulates allocation behavior,
    // we would expect the allocation size to be greater than 0.
    assert!(map.allocation_size() > 0);
}

#[test]
fn test_allocation_size_after_removal() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, 10);
    map.insert(2, 20);
    map.remove(&1);

    // The allocation size should still be > 0 after removal,
    // but the exact value depends on internal implementation.
    assert!(map.allocation_size() > 0);
}

#[test]
#[should_panic]
fn test_allocation_size_with_invalid_state() {
    struct FaultyAllocator;
    unsafe impl Allocator for FaultyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<i32, i32, DefaultHashBuilder, FaultyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: FaultyAllocator,
            marker: PhantomData,
        },
    };

    // Calling allocation_size when the allocator is in a faulty state.
    let _size = map.allocation_size();
}

