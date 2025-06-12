// Answer 0

#[test]
fn test_reserve() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;
    
    struct Alloc;
    unsafe impl Allocator for Alloc {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let mut map: HashMap<&str, i32, BuildHasherDefault<DefaultHasher>, Alloc> = HashMap {
        hash_builder: BuildHasherDefault::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: Alloc,
            marker: PhantomData,
        },
    };

    // Initial capacity check
    assert_eq!(map.capacity(), 0);
    
    // Reserve space for 10 elements
    map.reserve(10);
    
    // Assert that the capacity is now at least 10
    assert!(map.capacity() >= 10);
}

#[test]
#[should_panic]
fn test_reserve_exceeding_isize_max() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;
    
    struct Alloc;
    unsafe impl Allocator for Alloc {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, i32, BuildHasherDefault<DefaultHasher>, Alloc> = HashMap {
        hash_builder: BuildHasherDefault::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: Alloc,
            marker: PhantomData,
        },
    };

    // Panics when trying to reserve more than isize::MAX
    map.reserve(usize::MAX);
}

