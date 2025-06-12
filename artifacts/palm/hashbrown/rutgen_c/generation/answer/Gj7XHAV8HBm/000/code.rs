// Answer 0

#[test]
fn test_allocation_size_empty_set() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let set: HashSet<i32, DefaultHashBuilder, CustomAllocator> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder,
            table: RawTable::new(),
        },
    };

    assert_eq!(set.allocation_size(), 0);
}

#[test]
fn test_allocation_size_after_insertion() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut set: HashSet<i32, DefaultHashBuilder, CustomAllocator> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder,
            table: RawTable::new(),
        },
    };

    set.insert(1);
    set.insert(2);

    // Replace this with the actual expected allocation size after insertions.
    assert!(set.allocation_size() > 0);
}

#[test]
fn test_allocation_size_with_removal() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut set: HashSet<i32, DefaultHashBuilder, CustomAllocator> = HashSet {
        map: HashMap {
            hash_builder: DefaultHashBuilder,
            table: RawTable::new(),
        },
    };

    set.insert(1);
    set.insert(2);
    set.remove(&1);

    // The allocation size should remain greater than zero after a single removal.
    assert!(set.allocation_size() > 0);
}

