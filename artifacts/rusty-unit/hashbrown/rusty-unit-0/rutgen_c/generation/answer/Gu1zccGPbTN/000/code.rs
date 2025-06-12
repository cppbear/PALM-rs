// Answer 0

#[test]
fn test_reserve_initial_capacity() {
    struct DummyAllocator;
    
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    assert_eq!(map.table.table.capacity(), 0);
    map.reserve(10);
    assert!(map.table.table.capacity() >= 10);
}

#[test]
#[should_panic]
fn test_reserve_exceeding_isize_max() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    map.reserve(isize::MAX as usize + 1);
}

