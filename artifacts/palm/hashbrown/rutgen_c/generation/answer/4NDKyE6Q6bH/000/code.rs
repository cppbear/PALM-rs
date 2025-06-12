// Answer 0

#[test]
fn test_allocation_size_initial() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let hashmap: HashMap<i32, i32, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(), // Assuming a default constructor exists
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    assert_eq!(hashmap.allocation_size(), 0); // Assuming initial allocation size is 0
}

#[test]
fn test_allocation_size_after_insertions() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<i32, i32, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    hashmap.insert(1, 100); // Assuming insert function modifies internal allocation
    hashmap.insert(2, 200);

    assert!(hashmap.allocation_size() > 0); // Checking that some memory has been allocated
}

#[test]
fn test_allocation_size_after_removals() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<i32, i32, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    hashmap.insert(1, 100);
    hashmap.insert(2, 200);
    assert!(hashmap.allocation_size() > 0);

    hashmap.remove(&1); // Assuming remove function may affect internal allocation
    hashmap.remove(&2);

    assert_eq!(hashmap.allocation_size(), 0); // Assuming all memory is freed after removals
}

