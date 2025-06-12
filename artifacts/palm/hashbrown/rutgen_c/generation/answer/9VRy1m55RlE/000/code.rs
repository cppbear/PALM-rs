// Answer 0

#[test]
fn test_shrink_to_fit() {
    use core::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<i32, BuildHasherDefault<RandomState>, SimpleAllocator> = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    assert!(set.map.table.capacity() >= 100);
    set.shrink_to_fit();
    assert!(set.map.table.capacity() >= 2);
}

#[test]
fn test_shrink_to_fit_empty() {
    use core::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<i32, BuildHasherDefault<RandomState>, SimpleAllocator> = HashSet::new();
    assert!(set.map.table.capacity() == 0);
    set.shrink_to_fit();
    assert!(set.map.table.capacity() == 0);
}

